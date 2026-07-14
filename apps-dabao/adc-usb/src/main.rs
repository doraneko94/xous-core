fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let usb = usb_bao1x::UsbHid::new();

    std::thread::sleep(std::time::Duration::from_secs(1));

    let adc = bao1x_hal_service::Adc::new();

    unsafe { adc.enable_channel(bao1x_hal::udma::AdcExtChannel::Adc0) };

    loop {
        let raw = adc.read_raw(bao1x_hal::udma::AdcSource::Ext(bao1x_hal::udma::AdcExtChannel::Adc0), None);

        let voltage = bao1x_hal::udma::Adc::raw_to_voltage(raw);
        let message = format!("{}\r\n", voltage);
        usb.serial_send(message.as_bytes()).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
