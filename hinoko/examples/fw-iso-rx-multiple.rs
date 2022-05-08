// SPDX-License-Identifier: MIT

use hinoko::{traits::*, *};
use std::{sync, thread, time::Duration};

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
    const ISOC_CHANNELS: &[u8] = &[30, 31, 32, 33];
    const BYTES_PER_CHUNK: u32 = 4096;
    const CHUNKS_PER_BUFFER: u32 = 32;
    const ISOC_SYNC: u32 = 0;
    const CHUNKS_PER_INTERRUPT: u32 = 4;

    let isoc_tags: FwIsoCtxMatchFlag = FwIsoCtxMatchFlag::TAG0
        | FwIsoCtxMatchFlag::TAG1
        | FwIsoCtxMatchFlag::TAG2
        | FwIsoCtxMatchFlag::TAG3;
    let ir = FwIsoRxMultiple::new();
    if ir.allocate(PATH, ISOC_CHANNELS).is_err() {
        println!("Need to have access permission to {}.", PATH);
        return;
    }
    ir.map_buffer(BYTES_PER_CHUNK, CHUNKS_PER_BUFFER).unwrap();

    ir.connect_interrupted(|ir, count: u32| {
        let mut frames = [0; 4];

        (0..count).for_each(|i| {
            let data = ir.get_payload(i);

            frames.copy_from_slice(&data[0..4]);
            let iso_header = u32::from_le_bytes(frames);

            let payload_size = data.len() - 8;

            let end = data.len();
            frames.copy_from_slice(&data[(end - 4)..end]);
            let tstamp = u32::from_le_bytes(frames);

            println!("{:8x} {:8x} {:4}", tstamp, iso_header, payload_size);
        });
    });

    let src = ir.create_source().unwrap();
    let (dispatcher_cntr, th) = launch_dispatcher(&src);

    ir.start(None, ISOC_SYNC, isoc_tags, CHUNKS_PER_INTERRUPT)
        .unwrap();

    thread::sleep(DURATION);

    ir.stop();

    dispatcher_cntr.quit();
    th.join().unwrap();

    ir.unmap_buffer();
    ir.release();
}
