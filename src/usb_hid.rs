/*!
Implementation of the Device trait for USB connected lights.
*/

use crate::{Device, DeviceIdentifier, Pattern, SolidColor};
use hidapi::{HidApi, HidDevice};
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enables the discovery of the device using the USB HID descriptor.
///
#[allow(missing_debug_implementations)]
pub struct USBDeviceDiscovery {
    hid_api: HidApi,
}

///
/// The device identifier for a USB connected light.
///
#[derive(Clone, Debug)]
pub struct USBDeviceID(String);

///
/// The device implementation for a USB connected light.
///
#[allow(missing_debug_implementations)]
pub struct USBDevice<'a> {
    hid_device: HidDevice<'a>,
    id: USBDeviceID,
}

// ------------------------------------------------------------------------------------------------
// API Constants
// ------------------------------------------------------------------------------------------------

const LUXAFOR_VENDOR_ID: u16 = 0x04d8;
const LUXAFOR_PRODUCT_ID: u16 = 0xf372;

const MODE_SOLID: u8 = 1;
#[allow(dead_code)]
const MODE_FADE: u8 = 2;
const MODE_STROBE: u8 = 3;
#[allow(dead_code)]
const MODE_WAVE: u8 = 4;
const MODE_PATTERN: u8 = 6;

#[allow(dead_code)]
const LED_FRONT_TOP: u8 = 1;
#[allow(dead_code)]
const LED_FRONT_MIDDLE: u8 = 2;
#[allow(dead_code)]
const LED_FRONT_BOTTOM: u8 = 3;
#[allow(dead_code)]
const LED_BACK_TOP: u8 = 4;
#[allow(dead_code)]
const LED_BACK_MIDDLE: u8 = 5;
#[allow(dead_code)]
const LED_BACK_BOTTOM: u8 = 6;
#[allow(dead_code)]
const LED_FRONT_ALL: u8 = 65;
#[allow(dead_code)]
const LED_BACK_ALL: u8 = 66;
const LED_ALL: u8 = 255;

const PATTERN_LUXAFOR: u8 = 1;
const PATTERN_RANDOM_1: u8 = 2;
const PATTERN_RANDOM_2: u8 = 3;
const PATTERN_RANDOM_3: u8 = 4;
const PATTERN_RANDOM_4: u8 = 6;
const PATTERN_RANDOM_5: u8 = 7;
const PATTERN_POLICE: u8 = 5;
const PATTERN_RAINBOW_WAVE: u8 = 8;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for USBDeviceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DeviceIdentifier for USBDeviceID {}

// ------------------------------------------------------------------------------------------------

impl USBDeviceDiscovery {
    ///
    /// Construct a new discovery object, this initializes the USB HID interface and thus can fail.
    ///
    pub fn new() -> crate::error::Result<Self> {
        let hid_api = HidApi::new()?;
        Ok(Self { hid_api })
    }

    ///
    /// Return a device, if found, that corresponds to a Luxafor light.
    ///
    pub fn device(&self) -> crate::error::Result<USBDevice<'_>> {
        let result = self.hid_api.open(LUXAFOR_VENDOR_ID, LUXAFOR_PRODUCT_ID);
        match result {
            Ok(hid_device) => USBDevice::new(hid_device),
            Err(err) => {
                error!("Could not open HID device: {:?}", err);
                Err(crate::error::ErrorKind::DeviceNotFound.into())
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Device for USBDevice<'a> {
    fn id(&self) -> &dyn DeviceIdentifier {
        &self.id
    }

    fn turn_off(&self) -> crate::error::Result<()> {
        self.set_solid_color(
            SolidColor::Custom {
                red: 00,
                green: 00,
                blue: 00,
            },
            false,
        )
    }

    fn set_solid_color(&self, color: SolidColor, blink: bool) -> crate::error::Result<()> {
        info!("Setting the color of device '{}' to {}", self.id, color);
        let (r, g, b) = match color {
            SolidColor::Red => (255, 0, 0),
            SolidColor::Green => (0, 255, 0),
            SolidColor::Yellow => (255, 255, 0),
            SolidColor::Blue => (0, 0, 255),
            SolidColor::White => (255, 255, 255),
            SolidColor::Cyan => (0, 255, 255),
            SolidColor::Magenta => (255, 0, 255),
            SolidColor::Custom { red, green, blue } => (red, green, blue),
        };
        let mode = if blink { MODE_STROBE } else { MODE_SOLID };
        trace!("{} ({:#04x},{:#04x},{:#04x})", mode, r, g, b);
        let result = self.hid_device.write(&[mode, LED_ALL, r, g, b]);
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Could not write to HID device: {:?}", err);
                Err(crate::error::ErrorKind::InvalidRequest.into())
            }
        }
    }

    fn set_pattern(&self, pattern: Pattern) -> crate::error::Result<()> {
        info!("Setting the pattern of device '{}' to {}", self.id, pattern);
        let pattern = match pattern {
            Pattern::Police => PATTERN_POLICE,
            Pattern::TrafficLights => PATTERN_LUXAFOR,
            Pattern::Random(n) => match n {
                1 => PATTERN_RANDOM_1,
                2 => PATTERN_RANDOM_2,
                3 => PATTERN_RANDOM_3,
                4 => PATTERN_RANDOM_4,
                _ => PATTERN_RANDOM_5,
            },
            Pattern::Rainbow => PATTERN_RAINBOW_WAVE,
            Pattern::Sea => 9,
            Pattern::WhiteWave => 10,
            Pattern::Synthetic => 11,
        };
        let result = self.hid_device.write(&[MODE_PATTERN, LED_ALL, pattern]);
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Could not write to HID device: {:?}", err);
                Err(crate::error::ErrorKind::InvalidRequest.into())
            }
        }
    }
}

impl<'a> USBDevice<'a> {
    fn new(hid_device: HidDevice<'a>) -> crate::error::Result<USBDevice<'a>> {
        let id = USBDeviceID(format!(
            "{}::{}::{}",
            hid_device
                .get_manufacturer_string()
                .unwrap_or("<unknown>".to_string()),
            hid_device
                .get_product_string()
                .unwrap_or("<unknown>".to_string()),
            hid_device
                .get_serial_number_string()
                .unwrap_or("<unknown>".to_string())
        ));
        Ok(Self { hid_device, id })
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{Device, SolidColor};

    #[test]
    fn test_discovery() {
        let result = super::USBDeviceDiscovery::new();
        assert!(result.is_ok());
        let discovery = result.unwrap();

        let result = discovery.device();
        assert!(result.is_ok());
        let device = result.unwrap();
        println!("{}", device.id());

        let result = device.set_solid_color(SolidColor::Green, false);
        assert!(result.is_ok());
    }
}
