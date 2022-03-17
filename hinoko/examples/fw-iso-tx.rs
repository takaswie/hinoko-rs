// SPDX-License-Identifier: MIT
use hinoko::*;
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
    const ISOC_SPEED: FwScode = FwScode::S400;
    const ISOC_CHANNEL: u32 = 30;
    const CTX_HEADER_SIZE: u32 = 8;
    const MAXIMUM_BYTES_PER_PAYLOAD: u32 = 32;
    const PAYLOADS_PER_BUFFER: u32 = 32;
    const PAYLOADS_PER_INTERRUPT: u32 = 8;
    const ISOC_SYNC: u32 = 0;
    const ISOC_TAG: FwIsoCtxMatchFlag = FwIsoCtxMatchFlag::TAG0;

    let packet_count_cntr: sync::Arc<sync::Mutex<u32>> = sync::Arc::new(sync::Mutex::new(0));

    let it = FwIsoTx::new();
    if it
        .allocate(PATH, ISOC_SPEED, ISOC_CHANNEL, CTX_HEADER_SIZE)
        .is_err()
    {
        println!("Need to have access permission to {}.", PATH);
        return;
    }

    it.map_buffer(MAXIMUM_BYTES_PER_PAYLOAD, PAYLOADS_PER_BUFFER)
        .unwrap();

    let cntr = packet_count_cntr.clone();
    it.connect_interrupted(move |it, sec: u32, cycle: u32, header: &[u8], count: u32| {
        let my_header = [0, 1, 2, 3, 4, 5, 6, 7];
        let my_payload = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut frames = [0; 4];

        println!("sec: {}, cycle: {}, count: {}", sec, cycle, count);

        if let Ok(mut packet_count) = cntr.lock() {
            (0..count).for_each(|i| {
                let index = 4 * i as usize;
                frames.copy_from_slice(&header[index..(index + 4)]);
                let tstamp = u32::from_be_bytes(frames);
                println!("  {:2}: {:08x}", i, tstamp);

                *packet_count += 1;
                if *packet_count >= u32::MAX {
                    *packet_count %= PAYLOADS_PER_INTERRUPT;
                }
                let schedule_interrupt = *packet_count % PAYLOADS_PER_INTERRUPT == 0;

                it.register_packet(
                    ISOC_TAG,
                    ISOC_SYNC,
                    Some(&my_header),
                    Some(&my_payload),
                    schedule_interrupt,
                )
                .unwrap();
            });
        }
    });

    let src = it.create_source().unwrap();
    let (dispatcher_cntr, th) = launch_dispatcher(&src);

    let skip_count: u32 = cmp::min(PAYLOADS_PER_BUFFER / 2, PAYLOADS_PER_INTERRUPT * 2);
    (0..skip_count).for_each(|i| {
        let schedule_interrupt = i % PAYLOADS_PER_INTERRUPT == 0;
        it.register_packet(ISOC_TAG, ISOC_SYNC, None, None, schedule_interrupt)
            .unwrap();
    });
    *packet_count_cntr.lock().unwrap() = skip_count;

    it.start(None).unwrap();

    thread::sleep(DURATION);

    it.stop();

    dispatcher_cntr.quit();
    th.join().unwrap();

    it.unmap_buffer();
    it.release();
}
