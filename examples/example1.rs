fn main() {
    // create the device
    // where spi implements Transfer<u8, Error = SpiError>,
    // and cs implements OutputPin<Error = PinError>
    let mut accel = Lis2dw12::new(i2c, cs);

    // confirm that communication is working
    accel.check_who_am_i()?;

    // set up the device
    accel.set_operating_mode(OperatingMode::HighPerformance)?;
    accel.set_low_noise(true)?;
    accel.set_full_scale_selection(FullScaleSelection::PlusMinus2G)?; //precision of measure
    accel.set_output_data_rate(OutputDataRate::Hp100HzLp100Hz)?; // 100 Hz

    // get raw data
    let raw = accel.accel_raw()?;
    rprintln!("raw: {:?}", raw);
}
