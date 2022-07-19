#![no_std]
#![no_main]

mod usb;

use core::fmt::Write as _;
use core::panic::PanicInfo;

use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint as _;
use rp2040_hal as hal;
use rp2040_hal::{clocks::Clock as _, pac, pac::interrupt, sio::Sio, watchdog::Watchdog};
use usb_device;
use usb_device::bus::UsbBusAllocator;

use usb::usb_manager::UsbManager;

/// External high-speed crystal on the pico board is 12Mhz
pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[link_section = ".boot2"]
#[used]
/// Boot loader
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_MANAGER: Option<UsbManager> = None;

#[interrupt]
/// USB Interrupt handler
/// If USB_MANAGER is set, this shoots control over to its interrupt method
unsafe fn USBCTRL_IRQ() {
	match USB_MANAGER.as_mut() {
		Some(manager) => manager.interrupt(),
		None => (),
	};
}

#[panic_handler]
/// For when the program shits its pants
fn panic(panic_info: &PanicInfo) -> ! {
	if let Some(usb) = unsafe { USB_MANAGER.as_mut() } {
		writeln!(usb, "{}", panic_info).ok();
	}
	loop {}
}
#[cortex_m_rt::entry]
fn main() -> ! {
	// Set up hardware
	// If any of these fail, the whole thing should shit the bed
	let mut hardware = pac::Peripherals::take().unwrap();
	let core = pac::CorePeripherals::take().unwrap();
	let mut watchdog = Watchdog::new(hardware.WATCHDOG);
	let sio = Sio::new(hardware.SIO);

	let clocks = hal::clocks::init_clocks_and_plls(
		XOSC_CRYSTAL_FREQ,
		hardware.XOSC,
		hardware.CLOCKS,
		hardware.PLL_SYS,
		hardware.PLL_USB,
		&mut hardware.RESETS,
		&mut watchdog
	).ok().unwrap();

	let usb = unsafe {
		USB_BUS = Some(UsbBusAllocator::new(hal::usb::UsbBus::new(
			hardware.USBCTRL_REGS,
			hardware.USBCTRL_DPRAM,
			clocks.usb_clock,
			true,
			&mut hardware.RESETS,
		)));
		USB_MANAGER = Some(UsbManager::new(USB_BUS.as_ref().unwrap()));
		// Enable the USB interrupt
		pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
		USB_MANAGER.as_mut().unwrap()
	};

	let pins = hal::gpio::Pins::new(
		hardware.IO_BANK0,
		hardware.PADS_BANK0,
		sio.gpio_bank0,
		&mut hardware.RESETS,
	);

	let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
	let mut led_pin = pins.gpio25.into_push_pull_output();

	loop {
		led_pin.set_high().unwrap();
		write!(usb, "On\n").unwrap();
		delay.delay_ms(500);

		led_pin.set_low().unwrap();
		write!(usb, "Off\n").unwrap();
		delay.delay_ms(500);
	}
}
