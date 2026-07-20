use usb_bao1x::{
    UsbDeviceState,
    UsbHid,
    SERIAL_BINARY_BUFLEN,
};

const START_DELAY_MS: usize = 3000;
const TX_BLOCK_SIZE: usize = SERIAL_BINARY_BUFLEN;
#[allow(dead_code)]
const BLOCKS_PER_REPORT: u64 = 16_384;

//const SEND_INTERVAL_MS: usize = 1;

fn make_test_pattern() -> [u8; TX_BLOCK_SIZE] {
    let mut data = [0u8; TX_BLOCK_SIZE];

    let mut index = 0usize;
    while index < TX_BLOCK_SIZE {
        data[index] = index as u8;
        index += 1;
    }

    data
}

fn wait_until_usb_configured(
    usb: &UsbHid,
    timer: &ticktimer::Ticktimer,
) {
    while usb.status() != UsbDeviceState::Configured {
        timer.sleep_ms(10).ok();
    }
}

fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let timer = ticktimer::Ticktimer::new()
        .expect("could not connect to ticktimer");

    let usb = UsbHid::new();

    log::info!(
        "USB throughput test started: PID={}",
        xous::process::id()
    );

    log::info!("waiting for USB configuration");

    wait_until_usb_configured(&usb, &timer);

    log::info!(
        "USB configured; transmission starts in {} ms",
        START_DELAY_MS
    );

    timer.sleep_ms(START_DELAY_MS).ok();

    let tx_data = make_test_pattern();

    loop {
        let _ = usb.serial_send(&tx_data);
        //timer.sleep_ms(SEND_INTERVAL_MS).ok();
    }
}