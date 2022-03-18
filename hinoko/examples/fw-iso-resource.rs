// SPDX-License-Identifier: MIT

use hinoko::{traits::*, *};
use std::{sync, thread, time::Duration};

const PATH: &str = "/dev/fw1";
const CHANNEL: u8 = 32;
const TIMEOUT: u32 = 10;

fn launch_dispatcher(src: &glib::Source) -> (sync::Arc<glib::MainLoop>, thread::JoinHandle<()>) {
    let ctx = glib::MainContext::new();
    src.attach(Some(&ctx));

    let dispatcher = glib::MainLoop::new(Some(&ctx), false);
    let dispatcher_cntr = sync::Arc::new(dispatcher);
    let d = dispatcher_cntr.clone();
    let th = thread::spawn(move || {
        d.run();
        ()
    });

    loop {
        thread::sleep(Duration::from_millis(10));

        if dispatcher_cntr.is_running() {
            break;
        }
    }

    (dispatcher_cntr, th)
}

fn main() {
    // Test allocation bound to bus-generation.
    let res = FwIsoResource::new();
    if res.open(PATH, 0).is_err() {
        println!("Need to have access permission to /dev/fw1.");
        return;
    }

    let src = res.create_source().unwrap();
    let (dispatcher_cntr, th) = launch_dispatcher(&src);

    res.allocate_once_sync(&[CHANNEL.into()], TIMEOUT).unwrap();
    res.deallocate_once_sync(CHANNEL.into(), TIMEOUT).unwrap();

    src.destroy();
    dispatcher_cntr.quit();
    th.join().unwrap();

    // Test allocation maintained by Linux FireWire subsystem.
    let res = FwIsoResourceAuto::new();
    if res.open(PATH, 0).is_err() {
        println!("Need to have access permission to /dev/fw1.");
        return;
    }

    let src = res.create_source().unwrap();
    let (dispatcher_cntr, th) = launch_dispatcher(&src);

    res.allocate_sync(&[CHANNEL], TIMEOUT).unwrap();
    res.deallocate_sync().unwrap();

    src.destroy();
    dispatcher_cntr.quit();
    th.join().unwrap();
}
