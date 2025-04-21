#![no_main]
#![no_std]

//use atsamd_hal as hal;
use bsp::hal;
use cortex_m_rt::entry;
use feather as bsp;
use hal::prelude::*;

use hal::adc::{Adc, Gain, Reference, Resolution, SampleRate};
use hal::clock::GenericClockController;
use hal::pac::adc::inputctrl::Muxposselect;
use hal::pac::Peripherals;
use hal::prelude::*;

fn main() -> ! {
    // Initialize peripherals and clocks
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    // Configure ADC
    let mut adc = Adc::adc(peripherals.adc, &mut peripherals.pm, &mut clocks);

    // Set ADC configuration: 10-bit resolution, 1.0V reference, 1 sample
    adc.resolution(Resolution::_10bit);
    //adc.reference(Reference::Int1V0);
    adc.samples(SampleRate::_1);
    adc.gain(Gain::_1x); // Use 1x gain for internal channels

    // Read factory calibration data for temperature sensor
    let nvm = peripherals.nvmctrl;
    let calib_base = 0x00800080; // Software Calibration Area
    let temp_log_row = unsafe { core::ptr::read_volatile(calib_base as *const u64) };
    let room_temp_val = ((temp_log_row >> 12) & 0xFFF) as u16; // ROOM_TEMP_VAL_INT
    let hot_temp_val = ((temp_log_row >> 24) & 0xFFF) as u16; // HOT_TEMP_VAL_INT
    let room_temp = ((temp_log_row >> 36) & 0xFF) as u8; // ROOM_TEMP (25°C)
    let hot_temp = ((temp_log_row >> 44) & 0xFF) as u8; // HOT_TEMP (85°C)

    loop {
        // Helper function to read ADC for a given MUXPOS
        fn read_adc(adc: &mut Adc<hal::pac::Adc>, muxpos: Muxposselect) -> u16 {
            // Set input multiplexer
            adc.adc
                .inputctrl()
                .modify(|_, w| w.muxpos().variant(muxpos));
            while adc.adc.status().read().syncbusy().bit_is_set() {}
            // Perform conversion
            adc.adc.power_up();
            let result = adc.adc.convert();
            adc.adc.power_down();
            result
        }

        // Read temperature (MUXPOS = Temp, 0x18 or 24)
        let temp_raw = read_adc(&mut adc, Muxposselect::Temp);
        let temp_c = if room_temp_val != hot_temp_val {
            let temp_scaled = (temp_raw as i32 - room_temp_val as i32)
                * (hot_temp as i32 - room_temp as i32)
                / (hot_temp_val as i32 - room_temp_val as i32)
                + room_temp as i32;
            temp_scaled as f32
        } else {
            25.0 // Fallback
        };

        // Read VDDCORE (MUXPOS = Scaledcorevcc, 0x1A or 26, scaled by 1/4)
        let vddcore_raw = read_adc(&mut adc, Muxposselect::Scaledcorevcc);
        let vddcore_v = (vddcore_raw as f32 / 1023.0) * 1.0 * 4.0;

        // Read VDDANA (MUXPOS = Scalediovcc, 0x1B or 27, scaled by 1/6)
        let vddana_raw = read_adc(&mut adc, Muxposselect::Scalediovcc);
        let vddana_v = (vddana_raw as f32 / 1023.0) * 1.0 * 6.0;

        // Log results (replace with UART or networking)
        cortex_m::asm::bkpt(); // Use for debugging
                               // Example: println!("Temp: {:.1}°C, VDDCORE: {:.2}V, VDDANA: {:.2}V", temp_c, vddcore_v, vddana_v);

        // Simple delay (replace with PollingSysTick for precise timing)
        for _ in 0..1000000 {
            cortex_m::asm::nop();
        }
    }
}
