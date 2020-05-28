#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate glib_sys;
extern crate gobject_sys;
extern crate hinoko_sys;
extern crate libc;

pub use glib::Error;

mod auto;
pub use auto::*;

mod cycle_timer;
pub use cycle_timer::*;

mod fw_iso_ctx;
pub use fw_iso_ctx::*;

mod fw_iso_rx_single;
pub use fw_iso_rx_single::*;

mod fw_iso_rx_multiple;
pub use fw_iso_rx_multiple::*;

mod fw_iso_tx;
pub use fw_iso_tx::*;

#[cfg(test)]
mod tests {
    use glib;
    use FwIsoCtxExt;
    use FwIsoCtxMatchFlag;
    use FwIsoResource;
    use FwIsoResourceAuto;
    use FwIsoResourceAutoExt;
    use FwIsoResourceExt;
    use FwIsoRxMultiple;
    use FwIsoRxMultipleExt;
    use FwIsoRxMultipleExtManual;
    use FwIsoRxSingle;
    use FwIsoRxSingleExt;
    use FwIsoRxSingleExtManual;
    use FwIsoTx;
    use FwIsoTxExt;
    use FwIsoTxExtManual;
    use FwScode;

    fn launch_dispatcher(
        src: &glib::Source,
    ) -> (std::sync::Arc<glib::MainLoop>, std::thread::JoinHandle<()>) {
        let ctx = glib::MainContext::new();
        src.attach(Some(&ctx));

        let dispatcher = glib::MainLoop::new(Some(&ctx), false);
        let dispatcher_cntr = std::sync::Arc::new(dispatcher);
        let d = dispatcher_cntr.clone();
        let th = std::thread::spawn(move || {
            d.run();
            ()
        });

        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));

            if dispatcher_cntr.is_running() {
                break;
            }
        }

        (dispatcher_cntr, th)
    }

    #[test]
    fn iso_rx_single() {
        let ir = FwIsoRxSingle::new();
        if ir.allocate("/dev/fw1", 20, 8).is_err() {
            println!("Need to have access permission to /dev/fw1.");
            return;
        }
        ir.map_buffer(32, 32).unwrap();

        ir.connect_interrupted(|ir, sec: u32, cycle: u32, header: &[u8], count: u32| {
            let mut frames = [0; 4];

            println!("sec: {}, cycle: {}, count: {}", sec, cycle, count);

            for i in 0..count {
                let index = 8 * i as usize;
                frames.copy_from_slice(&header[index..(index + 4)]);
                let header1 = u32::from_be_bytes(frames);
                frames.copy_from_slice(&header[(index + 4)..(index + 8)]);
                let header2 = u32::from_be_bytes(frames);

                let payload = ir.get_payload(i).unwrap();
                println!(
                    "  {:2}: {:08x} {:08x} {:2}",
                    i,
                    header1,
                    header2,
                    payload.len()
                );
            }
        });

        let src = ir.create_source().unwrap();
        let (dispatcher_cntr, th) = launch_dispatcher(&src);

        ir.start(None, 0, FwIsoCtxMatchFlag::TAG1, 16).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        ir.stop();

        dispatcher_cntr.quit();
        th.join().unwrap();

        ir.unmap_buffer();
        ir.release();
    }

    #[test]
    fn iso_rx_multiple() {
        let ir = FwIsoRxMultiple::new();
        if ir.allocate("/dev/fw1", &[30, 31, 32, 33]).is_err() {
            println!("Need to have access permission to /dev/fw1.");
            return;
        }
        ir.map_buffer(4096, 32).unwrap();

        ir.connect_interrupted(|ir, count: u32| {
            let mut frames = [0; 4];

            for i in 0..count {
                let data = ir.get_payload(i).unwrap();

                frames.copy_from_slice(&data[0..4]);
                let iso_header = u32::from_le_bytes(frames);

                let payload_size = data.len() - 8;

                let end = data.len();
                frames.copy_from_slice(&data[(end - 4)..end]);
                let tstamp = u32::from_le_bytes(frames);

                println!("{:8x} {:8x} {:4}", tstamp, iso_header, payload_size);
            }
        });

        let src = ir.create_source().unwrap();
        let (dispatcher_cntr, th) = launch_dispatcher(&src);

        let tags = FwIsoCtxMatchFlag::TAG0
            | FwIsoCtxMatchFlag::TAG1
            | FwIsoCtxMatchFlag::TAG2
            | FwIsoCtxMatchFlag::TAG3;
        ir.start(None, 0, tags, 4).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        ir.stop();

        dispatcher_cntr.quit();
        th.join().unwrap();

        ir.unmap_buffer();
        ir.release();
    }

    #[test]
    fn isoc_tx() {
        let it = FwIsoTx::new();
        if it.allocate("/dev/fw1", FwScode::S400, 30, 8).is_err() {
            println!("Need to have access permission to /dev/fw1.");
            return;
        }

        it.map_buffer(32, 32).unwrap();

        it.connect_interrupted(|it, sec: u32, cycle: u32, header: &[u8], count: u32| {
            let my_header = [0, 1, 2, 3, 4, 5, 6, 7];
            let my_payload = [0, 1, 2, 3, 4, 5, 6, 7];
            let mut frames = [0; 4];

            println!("sec: {}, cycle: {}, count: {}", sec, cycle, count);

            for i in 0..count {
                let index = 4 * i as usize;
                frames.copy_from_slice(&header[index..(index + 4)]);
                let tstamp = u32::from_be_bytes(frames);
                println!("  {:2}: {:08x}", i, tstamp);

                it.register_packet(FwIsoCtxMatchFlag::TAG0, 0, &my_header, &my_payload)
                    .unwrap();
            }
        });

        let src = it.create_source().unwrap();
        let (dispatcher_cntr, th) = launch_dispatcher(&src);

        it.start(None, 16).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

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
