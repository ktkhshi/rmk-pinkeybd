#![no_std]
#![no_main]

mod keymap;
mod matrix;
mod vial;

use defmt::unwrap;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Flex, Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::mode::Async;
use embassy_nrf::peripherals::{RNG, USBD};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{bind_interrupts, rng, usb};
use nrf_mpsl::Flash;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use panic_probe as _;
use rmk::ble::BleTransport;
use rmk::config::{
    BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig,
};
use rmk::host::HostService;
use rmk::input_device::pmw3610::{BitBangSpiBus, Pmw3610, Pmw3610Config};
use rmk::input_device::pointing::{PointingDevice, PointingProcessor, PointingProcessorConfig};
use rmk::keyboard::Keyboard;
use rmk::split::PeripheralMatrixConfig;
use rmk::usb::UsbTransport;
use rmk::watchdog::Nrf52Watchdog;
use rmk::{KeymapData, initialize_keymap_and_storage, run_all};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    RNG => rng::InterruptHandler<RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler, usb::vbus_detect::InterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_scan()
        .support_central()
        .support_adv()
        .support_peripheral()
        .support_dle_peripheral()
        .support_dle_central()
        .support_phy_update_central()
        .support_phy_update_peripheral()
        .support_le_2m_phy()
        .central_count(1)?
        .peripheral_count(1)?
        .buffer_cfg(251, 251, 3, 3)?
        .build(p, rng, mpsl, mem)
}

fn ble_addr() -> [u8; 6] {
    let ficr = embassy_nrf::pac::FICR;
    let raw = (u64::from(ficr.deviceid(1).read()) << 32) | u64::from(ficr.deviceid(0).read());
    unwrap!((raw | 0x0000_c000_0000_0000).to_le_bytes()[..6].try_into())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);

    let mpsl_peripherals =
        mpsl::Peripherals::new(p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31);
    let lfclk = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    static SESSION_MEM: StaticCell<mpsl::SessionMem<1>> = StaticCell::new();
    let mpsl = MPSL.init(unwrap!(MultiprotocolServiceLayer::with_timeslots(
        mpsl_peripherals,
        Irqs,
        lfclk,
        SESSION_MEM.init(mpsl::SessionMem::new()),
    )));
    spawner.spawn(mpsl_task(&*mpsl).unwrap());

    let sdc_peripherals = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24,
        p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut sdc_mem = sdc::Mem::<6080>::new();
    let sdc = unwrap!(build_sdc(sdc_peripherals, &mut rng, mpsl, &mut sdc_mem));

    // Right side is the split central. Its keys occupy logical rows 6..10.
    let mut matrix = matrix::new::<6>([
        Flex::new(p.P0_03),
        Flex::new(p.P0_28),
        Flex::new(p.P0_29),
        Flex::new(p.P1_11),
        Flex::new(p.P1_12),
        Flex::new(p.P1_13),
    ]);
    // Right-side PMW3610: D5=SCLK, D4=SDIO, D9=NCS, D10=MOT.
    let pmw_spi = BitBangSpiBus::new(
        Output::new(p.P0_05, Level::High, OutputDrive::Standard),
        Flex::new(p.P0_04),
    );
    let pmw_cs = Output::new(p.P1_14, Level::High, OutputDrive::Standard);
    let pmw_motion = Some(Input::new(p.P1_15, Pull::Up));
    let mut trackball = PointingDevice::<Pmw3610<_, _, _>>::with_report_hz(
        0,
        pmw_spi,
        pmw_cs,
        pmw_motion,
        Pmw3610Config {
            res_cpi: 400,
            smart_mode: true,
            ..Default::default()
        },
        125,
    );

    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));
    let storage_config = StorageConfig {
        start_addr: 0xA0000,
        num_sectors: 6,
        ..Default::default()
    };
    let flash = Flash::take(mpsl, p.NVMC);
    let device_config = DeviceConfig {
        manufacturer: "Pinkeybd",
        product_name: "Pinkeybd RMK",
        ..DeviceConfig::default()
    };
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[]);
    let rmk_config = RmkConfig {
        device_config,
        storage_config,
        vial_config,
        ..Default::default()
    };
    let mut keymap_data = KeymapData::new_with_encoder(
        keymap::get_default_keymap(),
        keymap::get_default_encoder_map(),
    );
    let mut behavior_config = BehaviorConfig::default();
    let positional_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &rmk_config.storage_config,
        &mut behavior_config,
        &positional_config,
    )
    .await;

    let mut keyboard = Keyboard::new(&keymap);
    let host_service = HostService::new(&keymap, &rmk_config);
    let mut usb_transport =
        UsbTransport::new(driver, rmk_config.device_config).with_host_service(&host_service);
    let mut ble_transport = BleTransport::new(
        sdc,
        ble_addr(),
        rmk_config,
        [PeripheralMatrixConfig {
            rows: matrix::LOCAL_ROWS as u8,
            cols: matrix::COLS as u8,
            row_offset: 0,
            col_offset: 0,
        }],
    )
    .with_host_service(&host_service);
    // The PMW3610 is connected to this (central) half.
    let mut pointing_processor = PointingProcessor::new(
        &keymap,
        PointingProcessorConfig {
            invert_y: false,
            swap_xy: true,
            ..Default::default()
        },
    );
    let mut watchdog = Nrf52Watchdog::default_runner(p.WDT);

    run_all!(
        matrix,
        trackball,
        storage,
        usb_transport,
        ble_transport,
        pointing_processor,
        keyboard,
        watchdog
    )
    .await;
}
