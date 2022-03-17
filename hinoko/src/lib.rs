// SPDX-License-Identifier: MIT
mod auto;
mod cycle_timer;
mod fw_iso_ctx;
mod fw_iso_rx_multiple;
mod fw_iso_rx_single;
mod fw_iso_tx;

pub use crate::{
    auto::*, cycle_timer::*, fw_iso_ctx::*, fw_iso_rx_multiple::*, fw_iso_rx_single::*,
    fw_iso_tx::*, traits::*,
};
pub use ffi;

use glib::{signal::*, translate::*, Cast, IsA, SignalHandlerId, StaticType, Value};

use libc::c_uint;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{cmp, sync, thread, time::Duration};

    const PATH: &str = "/dev/fw1";
    const DURATION: Duration = Duration::from_secs(1);

    fn launch_dispatcher(
        src: &glib::Source,
    ) -> (sync::Arc<glib::MainLoop>, thread::JoinHandle<()>) {
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

    #[test]
    fn iso_rx_single() {
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

                    let payload = ir.get_payload(i).unwrap();
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

    #[test]
    fn iso_rx_multiple() {
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
                let data = ir.get_payload(i).unwrap();

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

    #[test]
    fn isoc_tx() {
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

    #[test]
    fn iso_resource() {
        // Test allocation bound to bus-generation.
        let res = FwIsoResource::new();
        if res.open("/dev/fw1", 0).is_err() {
            println!("Need to have access permission to /dev/fw1.");
            return;
        }

        let src = res.create_source().unwrap();
        let (dispatcher_cntr, th) = launch_dispatcher(&src);

        res.allocate_once_sync(&[32], 10).unwrap();
        res.deallocate_once_sync(32, 10).unwrap();

        src.destroy();
        dispatcher_cntr.quit();
        th.join().unwrap();

        // Test allocation maintained by Linux FireWire subsystem.
        let res = FwIsoResourceAuto::new();
        if res.open("/dev/fw1", 0).is_err() {
            println!("Need to have access permission to /dev/fw1.");
            return;
        }

        let src = res.create_source().unwrap();
        let (dispatcher_cntr, th) = launch_dispatcher(&src);

        res.allocate_sync(&[32], 10).unwrap();
        res.deallocate_sync().unwrap();

        src.destroy();
        dispatcher_cntr.quit();
        th.join().unwrap();
    }
}
