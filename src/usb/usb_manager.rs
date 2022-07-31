use rp2040_hal as hal;
use usb_device;
use usb_device::{
	bus::UsbBusAllocator,
	device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::SerialPort;

pub struct UsbManager {
	device: UsbDevice<'static, hal::usb::UsbBus>,
	serial: SerialPort<'static, hal::usb::UsbBus>,
}

impl UsbManager {
	pub fn new(usb_bus: &'static UsbBusAllocator<hal::usb::UsbBus>) -> Self {
		let serial = usbd_serial::SerialPort::new(usb_bus);

		let device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x2E8A, 0x000a))
			.manufacturer("Raspberry Pi")
			.product("Pico")
			.serial_number("TEST")
			.device_class(2)
			.device_protocol(1)
			.build();

		UsbManager { device, serial }
	}

	pub unsafe fn interrupt(&mut self) {
		if self.device.poll(&mut [&mut self.serial]) {}
	}
}

impl core::fmt::Write for UsbManager {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		// For now, just ignore any write errors to allow a semi-stable connection
		// After some amount of data written to the buffer with no place to go,
		// this throws an error and kills everything
		self.serial.write(s.as_bytes());

		Ok(())
	}
}

