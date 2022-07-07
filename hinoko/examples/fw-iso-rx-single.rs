// SPDX-License-Identifier: MIT

use hinoko::{prelude::*, *};
use std::{cmp, sync, thread, time::Duration};

const PATH: &str = "/dev/fw1";
const DURATION: Duration = Duration::from_secs(1);

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
    const ISOC_CHANNEL: u32 = 20;
    const CTX_HEADER_SIZE: u32 = 8;
    const MAXIMUM_BYTES_PER_PAYLOAD: u32 = 32;
    const PAYLOADS_PER_BUFFER: u32 = 32;
    const PAYLOADS_PER_INTERRUPT: u32 = 8;
    const ISOC_SYNC: u32 = 0;
    const ISOC_TAG: FwIsoCtxMatchFlag = FwIsoCtxMatchFlag::TAG1;

    let packet_count_cntr: sync::Arc<sync::Mutex<u32>> = sync::Arc::new(sync::Mutex::new(0));

    let ir = FwIsoRxSingle::new();
    if ir.allocate(PATH, ISOC_CHANNEL, CTX_HEADER_SIZE).is_err() {
        println!("Need to have access permission to {}.", PATH);
        return;
    }
    ir.map_buffer(MAXIMUM_BYTES_PER_PAYLOAD, PAYLOADS_PER_BUFFER)
        .unwrap();

    let cntr = packet_count_cntr.clone();
    ir.connect_interrupted(move |ir, sec: u32, cycle: u32, header: &[u8], count: u32| {
        let mut frames = [0; 4];

        println!("sec: {}, cycle: {}, count: {}", sec, cycle, count);

        if let Ok(mut packet_count) = cntr.lock() {
            (0..count).for_each(|i| {
                let mut ctx_header: [u32; 2] = [0; 2];
                let pos: usize = (CTX_HEADER_SIZE * i) as usize;
                frames.copy_from_slice(&header[pos..(pos + 4)]);
                ctx_header[0] = u32::from_be_bytes(frames);
                frames.copy_from_slice(&header[(pos + 4)..(pos + 8)]);
                ctx_header[1] = u32::from_be_bytes(frames);

                let payload = ir.get_payload(i);
                println!(
                    "  {:2}: {:08x} {:08x} {:2}",
                    i,
                    ctx_header[0],
                    ctx_header[1],
                    payload.len()
                );

                *packet_count += 1;
                if *packet_count >= u32::MAX {
                    *packet_count %= PAYLOADS_PER_INTERRUPT;
                }
                let schedule_interrupt = *packet_count % PAYLOADS_PER_INTERRUPT == 0;
                ir.register_packet(schedule_interrupt).unwrap();
            });
        }
    });

    let src = ir.create_source().unwrap();
    let (dispatcher_cntr, th) = launch_dispatcher(&src);

    let init_count: u32 = cmp::min(PAYLOADS_PER_BUFFER / 2, PAYLOADS_PER_INTERRUPT * 2);
    (0..init_count).for_each(|i| {
        let schedule_interrupt = i % PAYLOADS_PER_INTERRUPT == 0;
        ir.register_packet(schedule_interrupt).unwrap();
    });
    *packet_count_cntr.lock().unwrap() = init_count;

    ir.start(None, ISOC_SYNC, ISOC_TAG).unwrap();

    thread::sleep(DURATION);

    ir.stop();

    dispatcher_cntr.quit();
    th.join().unwrap();

    ir.unmap_buffer();
    ir.release();
}
