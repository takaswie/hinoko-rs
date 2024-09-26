// SPDX-License-Identifier: MIT
use glib::{prelude::*, *};
use hinoko::{prelude::*, *};

use std::{
    cell::RefCell,
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration,
};
use std::rc::Rc;

const PATH: &str = "/dev/fw1";
const CHANNEL: u8 = 32;
const BYTES_PER_PAYLOAD: u32 = 120;
const SCODE: FwScode = FwScode::S400;
const TIMEOUT_MS: u32 = 10;

struct ResourceData(Result<(u32, u32), glib::Error>);

impl Default for ResourceData {
    fn default() -> Self {
        Self(Ok(Default::default()))
    }
}

#[derive(Default)]
struct ResourceState(RefCell<(bool, ResourceData, Option<Waker>)>);

impl Future for &ResourceState {
    type Output = ResourceData;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        // The borrow checker detects no issue sicne a single GLib::MainContext executes both the
        // call and the allocated/deallocated closures sequentially. Nevertheless just for my safe.
        if let Ok(mut data) = self.0.try_borrow_mut() {
            if data.0 {
                Poll::Ready(mem::take(&mut data.1))
            } else {
                data.2 = Some(ctx.waker().clone());
                Poll::Pending
            }
        } else {
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Both FwIsoResourceExt::connect_allocated() and FwIsoResourceExt::connect_deallocated() require
// the same type of trait for their signal handler.
fn create_resource_signal_handler<T: FwIsoResourceExt>(
    state: &Rc<ResourceState>,
) -> impl Fn(&T, u32, u32, Option<&glib::Error>) -> () {
    let s = state.clone();

    move |_res, channel, bandwidth, error| {
        // The borrow checker detects no issue sicne a single GLib::MainContext executes both the
        // call and Future::poll() sequentially. Nevertheless just for my ease.
        if let Ok(mut state) = s.0.try_borrow_mut() {
            state.0 = true;
            let data = if let Some(err) = error {
                Err(err.clone())
            } else {
                Ok((channel, bandwidth))
            };
            state.1 = ResourceData(data);

            if let Some(waker) = &state.2 {
                waker.wake_by_ref();
            }
        }
    }
}

// The variation of Hinoko::FwIsoResourceExt::allocate_wait() for asynchronous runtime.
async fn allocate_wait_async<T: FwIsoResourceExt>(
    res: &T,
    channel_candidates: &[u8],
    bandwidth: u32,
    timeout_ms: u32,
) -> Result<(), glib::Error> {
    let state = Rc::new(ResourceState::default());

    let handler_id = res.connect_allocated(create_resource_signal_handler(&state));

    let future =
        glib::future_with_timeout(Duration::from_millis(timeout_ms.into()), state.as_ref());
    res.allocate(channel_candidates, bandwidth)?;

    let result = future.await;

    res.disconnect(handler_id);

    result
        .map_err(|timeout_error| {
            let msg = format!("{}", timeout_error);
            glib::Error::new(FwIsoResourceError::Timeout, &msg)
        })
        .and_then(|data| data.0.map(|_| ()))
}

// The variation of Hinoko::FwIsoResourceOnceExt::deallocate_wait() for asynchronous runtime.
async fn once_deallocate_wait_async<T: FwIsoResourceExt + FwIsoResourceOnceExt>(
    res: &T,
    channel: u32,
    bandwidth: u32,
    timeout_ms: u32,
) -> Result<(), glib::Error> {
    let state = Rc::new(ResourceState::default());

    let handler_id = res.connect_deallocated(create_resource_signal_handler(&state));

    let future =
        glib::future_with_timeout(Duration::from_millis(timeout_ms.into()), state.as_ref());
    res.deallocate(channel, bandwidth)?;

    let result = future.await;

    res.disconnect(handler_id);

    result
        .map_err(|timeout_error| {
            let msg = format!("{}", timeout_error);
            glib::Error::new(FwIsoResourceError::Timeout, &msg)
        })
        .and_then(|data| data.0.map(|_| ()))
}

// The variation of Hinoko::FwIsoResourceAutoExt::deallocate_wait() for asynchronous runtime.
async fn auto_deallocate_wait_async<T: FwIsoResourceExt + FwIsoResourceAutoExt>(
    res: &T,
    timeout_ms: u32,
) -> Result<(), glib::Error> {
    let state = Rc::new(ResourceState::default());

    let handler_id = res.connect_deallocated(create_resource_signal_handler(&state));

    let future =
        glib::future_with_timeout(Duration::from_millis(timeout_ms.into()), state.as_ref());
    res.deallocate()?;

    let result = future.await;

    res.disconnect(handler_id);

    result
        .map_err(|timeout_error| {
            let msg = format!("{}", timeout_error);
            glib::Error::new(FwIsoResourceError::Timeout, &msg)
        })
        .and_then(|data| data.0.map(|_| ()))
}

async fn async_main_once(res: FwIsoResourceOnce, ctx: &MainContext) -> Result<(), glib::Error> {
    let src = res.create_source()?;
    let _ = src.attach(Some(&ctx));

    let channels = [CHANNEL];
    let bandwidth = FwIsoResource::calculate_bandwidth(BYTES_PER_PAYLOAD, SCODE);

    allocate_wait_async(&res, &channels, bandwidth, TIMEOUT_MS).await?;

    once_deallocate_wait_async(&res, CHANNEL.into(), bandwidth, TIMEOUT_MS).await
}

async fn async_main_auto(res: FwIsoResourceAuto, ctx: &MainContext) -> Result<(), glib::Error> {
    let src = res.create_source()?;
    let _ = src.attach(Some(&ctx));

    let channels = [CHANNEL];
    let bandwidth = FwIsoResource::calculate_bandwidth(BYTES_PER_PAYLOAD, SCODE);

    allocate_wait_async(&res, &channels, bandwidth, TIMEOUT_MS).await?;

    auto_deallocate_wait_async(&res, TIMEOUT_MS).await
}

fn main() {
    let ctx = MainContext::default();

    // Test allocation bound to bus-generation.
    let res = FwIsoResourceOnce::new();
    if res.open(PATH, 0).is_err() {
        println!("Need to have access permission to {}", PATH);
        return;
    }
    ctx.block_on(async_main_once(res, &ctx)).unwrap();

    // Test allocation maintained by Linux FireWire subsystem.
    let res = FwIsoResourceAuto::new();
    if res.open(PATH, 0).is_err() {
        println!("Need to have access permission to {}", PATH);
        return;
    }
    ctx.block_on(async_main_auto(res, &ctx)).unwrap();
}
