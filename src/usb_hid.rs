/*!
Implementation of the Device trait for USB connected lights.

# Specification

The identifiers used to discover the device using the HID API are as follows:

1. The vendor identifier (VID) is `0x04D8`.
1. The product identifier (PID) is `0xF372`.

The following command groups exist for controlling the lights. Note that byte 0, the USB HID _report
identifier_, must always be set to `0x00`. Also, trailing `0x00` values need not be written.

| Command Group  | 0      | 1      | 2      | 3      | 4      | 5      | 6      | 7      | 8      |
|----------------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
| Simple         | `0x00` | `0x00` | COLOR* | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` |
| Solid          | `0x00` | `0x01` | LED    | RED    | GREEN  | BLUE   | `0x00` | `0x00` | `0x00` |
| Fade           | `0x00` | `0x02` | LED    | RED    | GREEN  | BLUE   | TIME   | `0x00` | `0x00` |
| Strobe         | `0x00` | `0x03` | LED    | RED    | GREEN  | BLUE   | SPEED  | `0x00` | REPEAT |
| Wave           | `0x00` | `0x04` | WTYPE  | RED    | GREEN  | BLUE   | `0x00` | REPEAT | SPEED  |
| Pattern        | `0x00` | `0x06` | PTYPE  | REPEAT | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` |
| Productivity   | `0x00` | `0x0A` | COLOR  | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` |
| Get Ver/Serial | `0x00` | `0x80` | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` | `0x00` |

## Notes

1. The values for LED, COLOR, WTYPE, and PTYPE, are shown in the corresponding tables below.
1. The values for RED, GREEN, BLUE, are `0..255` and correspond to standard RGB color.
1. The value for TIME is the number of seconds to fade from the current color to the new color specified.
1. The value for SPEED is the time to cycle through the change.
1. The value of REPEAT is the number of times to repeat the wave or pattern.
1. The response data for the get version and serial number command group is described below.

## LED values

| Value        | Addressed LED      |
|--------------|--------------------|
| `0x01..0x06` | Specific LED       |
| `0x41`       | all back LEDs      |
| `0x42`       | all front LEDs     |
| `0xFF`       | all LEDs one color |

The addressable LEDs on the _Flag_ USB product are as follows, shown in vertical orientation:

| Back | Front |
|------|-------|
| 6    | 3     |
| 5    | 2     |
| 4    | 1     |

## COLOR values

1. For the command group _Productivity_ all the values below are valid.
1. For the command group _Simple_ the values 'E' and 'D' are not valid.

| Letter | Value  | Color     |
|--------|--------|-----------|
| 'E'    | `0x45` | _Enable_  |
| 'D'    | `0x44` | _Disable_ |
| 'R'    | `0x52` | Red       |
| 'G'    | `0x47` | Green     |
| 'B'    | `0x42` | Blue      |
| 'C'    | `0x43` | Cyan      |
| 'M'    | `0x4D` | Magenta   |
| 'Y'    | `0x59` | Yellow    |
| 'W'    | `0x57` | White     |
| 'O'    | `0x4F` | Off       |

## WTYPE values

| Value  | Pattern            |
|--------|--------------------|
| `0x01` | Short              |
| `0x02` | Long               |
| `0x03` | Overlapping  Short |
| `0x04` | Overlapping  Long  |
| `0x05` | ?                  |

1. Luxafor describe wave type as a value `0x01..0x05` and yet there seems to be no description of `0x05` anywhere.

## PTYPE values

| Value  | Pattern      | Windows Only |
|--------|--------------|--------------|
| `0x00` | ?            | Unknown      |
| `0x01` | Luxafor/Traffic Lights | No           |
| `0x02` | Random 1     | No           |
| `0x03` | Random 2     | No           |
| `0x04` | Random 3     | No           |
| `0x05` | Police       | No           |
| `0x06` | Random 4     | No           |
| `0x07` | Random 5     | No           |
| `0x08` | Rainbow Wave | Yes          |

1. Luxafor describe pattern type as a value `0x00..0x08` and yet there seems to be no description of `0x00` anywhere.

## Version/Serial response

| 0      | 1          | 2           | 3          |
|--------|------------|-------------|------------|
| `0x80` | FW Version | Serial High | Serial Low |

The serial number is returned as a pair, (high,low) bytes.

*/

use crate::{Device, Pattern, SolidColor};
use hidapi::{HidApi, HidDevice};

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
/// The device implementation for a USB connected light.
///
#[allow(missing_debug_implementations)]
pub struct USBDevice<'a> {
    hid_device: HidDevice<'a>,
    id: String,
}

// ------------------------------------------------------------------------------------------------
// API Constants
// ------------------------------------------------------------------------------------------------

const LUXAFOR_VENDOR_ID: u16 = 0x04d8;
const LUXAFOR_PRODUCT_ID: u16 = 0xf372;

const HID_REPORT_ID: u8 = 0;

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
#[cfg(target_os = "windows")]
const PATTERN_RAINBOW_WAVE: u8 = 8;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
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
    fn id(&self) -> String {
        self.id.clone()
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
        self.write(&[HID_REPORT_ID, mode, LED_ALL, r, g, b])
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
            #[cfg(target_os = "windows")]
            Pattern::Rainbow => PATTERN_RAINBOW_WAVE,
            #[cfg(target_os = "windows")]
            Pattern::Sea => 9,
            #[cfg(target_os = "windows")]
            Pattern::WhiteWave => 10,
            #[cfg(target_os = "windows")]
            Pattern::Synthetic => 11,
        };
        self.write(&[HID_REPORT_ID, MODE_PATTERN, pattern, 255])
    }
}

impl<'a> USBDevice<'a> {
    fn new(hid_device: HidDevice<'a>) -> crate::error::Result<USBDevice<'a>> {
        let id = format!(
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
        );
        Ok(Self { hid_device, id })
    }

    fn write(&self, buffer: &[u8]) -> crate::error::Result<()> {
        trace!(
            "writing [{:?}]",
            buffer
                .iter()
                .map(|b| format!("{:#04x}", b))
                .collect::<Vec<String>>()
                .join(", ")
        );
        let result = self.hid_device.write(buffer);
        match result {
            Ok(bytes_written) => if bytes_written == buffer.len() {
                Ok(())
            } else {
                error!("Bytes written, {}, did not match buffer length {}", bytes_written, buffer.len());
                Err(crate::error::ErrorKind::InvalidRequest.into())
            }
            Err(err) => {
                error!("Could not write to HID device: {:?}", err);
                Err(crate::error::ErrorKind::InvalidRequest.into())
            }
        }
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
        if result.is_ok() {
            let discovery = result.unwrap();

            let result = discovery.device();
            assert!(result.is_ok());
            let device = result.unwrap();
            println!("{}", device.id());

            let result = device.set_solid_color(SolidColor::Green, false);
            assert!(result.is_ok());
        }
    }
}
