/*!
Library, and CLI, for [Luxafor](https://luxafor.com/products/) lights via either USB or webhooks.

The main entry point for clients is the trait [Device](trait.Device.html) that has implementations
for USB connected devices such as the [flag](https://luxafor.com/flag-usb-busylight-availability-indicator/)
as well as webhooks for both the flag and [bluetooth](https://luxafor.com/bluetooth-busy-light-availability-indicator/)
lights.

Each connection has its own discovery or connection methods but will provide a `Device` implementation
for the manipulation of the light state.

# API Examples

The following example shows a function that sets the light to a solid red color. It demonstrates
the use of a USB connected device.

```rust,ignore
use luxafor::usb_hid::USBDeviceDiscovery;
use luxafor::{Device, SolidColor, TargetedDevice};
use luxafor::error::Result;

fn set_do_not_disturb() -> Result<()> {
    let discovery = USBDeviceDiscovery::new()?;
    let device = discovery.device()?;
    println!("USB device: '{}'", device.id());
    device.set_specific_led(SpecificLED::AllFront);
    device.set_solid_color(SolidColor::Red, false)
}
```

The following shows the same function but using the webhook connection. Note that the webhook API
is more limited in the features it exposes; it does not support `set_specific_led` for a start.

```rust,ignore
use luxafor::webhook::new_device_for;
use luxafor::{Device, SolidColor};
use luxafor::error::Result;

fn set_do_not_disturb(device_id: &str) -> Result<()> {
    let device = new_device_for(device_id)?;
    println!("Webhook device: '{}'", device.id());
    device.set_solid_color(SolidColor::Red, false)
}
```

# CLI Examples

The following shows the command line tool setting the color to red.

```bash
❯ lux -d 2a0f2c73b72 solid red
```

The following shows the command line tool setting the color to a blinking green. This example uses the environment
variable `LUX_DEVICE` to save repeating the device identifier on each call.

```bash
❯ export LUX_DEVICE=2a0f2c73b72
❯ lux blink green
```

The following shows the command line tool turning the light off.

```bash
❯ lux -vvv -d 2a0f2c73b72 off
 INFO  luxafor > Setting the color of device '2a0f2c73b72e' to 000000
 INFO  luxafor > call successful
```

The following shows the how to set USB connected lights.

```bash
❯ lux -d usb solid red
```

# Features

* **command-line**; provides the command line tool `lux`, it is not on by default for library clients.
* **usb**; provides access to USB connected devices.
* **webhook** (default); provides access to USB, or Bluetooth, devices via webhooks.

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

#[macro_use]
extern crate error_chain;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A color that the light can be set to.
///
#[derive(Clone, Debug)]
pub enum SolidColor {
    /// A preset color
    Red,
    /// A preset color
    Green,
    /// A preset color
    Yellow,
    /// A preset color
    Blue,
    /// A preset color
    White,
    /// A preset color
    Cyan,
    /// A preset color
    Magenta,
    /// A custom color using standard RGB values
    Custom {
        /// The _red_ channel
        red: u8,
        /// The _green_ channel
        green: u8,
        /// The _blue_ channel
        blue: u8,
    },
}

///
/// Waves produce a pattern that starts at the bottom of the light, fills the light and then
/// fades out at the top.
///
#[derive(Clone, Debug)]
pub enum Wave {
    /// A short transition, completed before the next wave starts.
    Short,
    /// A long transition, completed before the next wave starts.
    Long,
    /// A short transition, which _does not_ complete before the next wave starts.
    OverlappingShort,
    /// A long transition, which _does not_ complete before the next wave starts.
    OverlappingLong,
}

///
/// A pattern the light can be set to show.
///
#[derive(Clone, Debug)]
pub enum Pattern {
    /// A preset pattern that cycles between red and blue.
    Police,
    /// A preset pattern that cycles between green,. yellow, and red.
    TrafficLights,
    /// Preset random patterns
    Random(u8),
    /// A preset pattern
    #[cfg(target_os = "windows")]
    Rainbow,
    /// A preset pattern
    #[cfg(target_os = "windows")]
    Sea,
    /// A preset pattern
    #[cfg(target_os = "windows")]
    WhiteWave,
    /// A preset pattern
    #[cfg(target_os = "windows")]
    Synthetic,
}

///
/// A trait implemented by different access methods to control a light.
///
pub trait Device {
    ///
    /// Return the identifier for the device.
    ///
    fn id(&self) -> String;

    ///
    /// Turn the light off.
    ///
    fn turn_off(&self) -> error::Result<()>;

    ///
    /// Set the light to a continuous solid color.
    ///
    fn set_solid_color(&self, color: SolidColor) -> error::Result<()>;

    ///
    /// Set the light to fade from its current color to a new one.
    ///
    fn set_fade_to_color(&self, color: SolidColor, fade_duration: u8) -> error::Result<()>;

    ///
    /// Strobe the light, this will dim and brighten the same color.
    ///
    fn set_color_strobe(
        &self,
        color: SolidColor,
        strobe_speed: u8,
        repeat_count: u8,
    ) -> error::Result<()>;

    ///
    /// Set the light to repeat one of a pre-defined set of wave patterns.
    ///
    fn set_color_wave(
        &self,
        color: SolidColor,
        wave_pattern: Wave,
        wave_speed: u8,
        repeat_count: u8,
    ) -> error::Result<()>;

    ///
    /// Set the light to repeat one of a pre-defined set of patterns.
    ///
    fn set_pattern(&self, pattern: Pattern, repeat_count: u8) -> error::Result<()>;
}

///
/// Denotes which LED in the light should be the target of any device operations.
///
#[derive(Clone, Debug)]
pub enum SpecificLED {
    /// All supported LEDs
    All,
    /// Only the LEDs on the front (tab) of the light
    AllFront,
    /// Only the LEDs on the back of the light
    AllBack,
    /// Only one specific LED (value: 1..6)
    Number(u8),
}

///
/// Extension trait to allow targeting specific LEDs on the device.
///
pub trait TargetedDevice: Device {
    /// Set the LED to be used for future operations.
    fn set_specific_led(&mut self, led: SpecificLED) -> error::Result<()>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SolidColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn to_hex(v: &u8) -> String {
            format!("{:#04x}", v)[2..].to_string()
        }
        write!(
            f,
            "{}",
            match self {
                SolidColor::Red => "red".to_string(),
                SolidColor::Green => "green".to_string(),
                SolidColor::Yellow => "yellow".to_string(),
                SolidColor::Blue => "blue".to_string(),
                SolidColor::White => "white".to_string(),
                SolidColor::Cyan => "cyan".to_string(),
                SolidColor::Magenta => "magenta".to_string(),
                SolidColor::Custom { red, green, blue } =>
                    format!("{}{}{}", to_hex(red), to_hex(green), to_hex(blue)),
            }
        )
    }
}

impl FromStr for SolidColor {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "red" => Ok(SolidColor::Red),
            "green" => Ok(SolidColor::Green),
            "yellow" => Ok(SolidColor::Yellow),
            "blue" => Ok(SolidColor::Blue),
            "white" => Ok(SolidColor::White),
            "cyan" => Ok(SolidColor::Cyan),
            "magenta" => Ok(SolidColor::Magenta),
            _ => {
                if s.len() == 6 && s.chars().all(|c| c.is_ascii_hexdigit()) {
                    Ok(SolidColor::Custom {
                        red: u8::from_str_radix(&s[0..1], 16)?,
                        green: u8::from_str_radix(&s[2..3], 16)?,
                        blue: u8::from_str_radix(&s[4..5], 16)?,
                    })
                } else {
                    Err(error::ErrorKind::InvalidColor.into())
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Wave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Wave::Short => "short",
                Wave::Long => "long",
                Wave::OverlappingShort => "overlapping short",
                Wave::OverlappingLong => "overlapping long",
            }
        )
    }
}

impl FromStr for Wave {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "short" => Ok(Wave::Short),
            "long" => Ok(Wave::Long),
            "overlapping short" => Ok(Wave::OverlappingShort),
            "overlapping long" => Ok(Wave::OverlappingLong),
            _ => Err(error::ErrorKind::InvalidPattern.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pattern::Police => "police".to_string(),
                Pattern::TrafficLights => "traffic lights".to_string(),
                Pattern::Random(n) => format!("random {}", n),
                #[cfg(target_os = "windows")]
                Pattern::Rainbow => "rainbow".to_string(),
                #[cfg(target_os = "windows")]
                Pattern::Sea => "sea".to_string(),
                #[cfg(target_os = "windows")]
                Pattern::WhiteWave => "white wave".to_string(),
                #[cfg(target_os = "windows")]
                Pattern::Synthetic => "synthetic".to_string(),
            }
        )
    }
}

impl FromStr for Pattern {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "police" => Ok(Pattern::Police),
            "traffic lights" => Ok(Pattern::TrafficLights),
            "random 1" => Ok(Pattern::Random(1)),
            "random 2" => Ok(Pattern::Random(2)),
            "random 3" => Ok(Pattern::Random(3)),
            "random 4" => Ok(Pattern::Random(4)),
            "random 5" => Ok(Pattern::Random(5)),
            #[cfg(target_os = "windows")]
            "rainbow" => Ok(Pattern::Rainbow),
            #[cfg(target_os = "windows")]
            "sea" => Ok(Pattern::Sea),
            #[cfg(target_os = "windows")]
            "white wave" => Ok(Pattern::WhiteWave),
            #[cfg(target_os = "windows")]
            "synthetic" => Ok(Pattern::Synthetic),
            _ => Err(error::ErrorKind::InvalidPattern.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SpecificLED {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpecificLED::All => "all".to_string(),
                SpecificLED::AllFront => "front".to_string(),
                SpecificLED::AllBack => "back".to_string(),
                SpecificLED::Number(n) => n.to_string(),
            }
        )
    }
}

impl FromStr for SpecificLED {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "all" => Ok(SpecificLED::All),
            "front" => Ok(SpecificLED::AllFront),
            "back" => Ok(SpecificLED::AllBack),
            "1" => Ok(SpecificLED::Number(1)),
            "2" => Ok(SpecificLED::Number(2)),
            "3" => Ok(SpecificLED::Number(3)),
            "4" => Ok(SpecificLED::Number(4)),
            "5" => Ok(SpecificLED::Number(5)),
            "6" => Ok(SpecificLED::Number(6)),
            _ => Err(error::ErrorKind::InvalidLED.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

///
/// Error handling types.
///
#[allow(missing_docs)]
pub mod error {
    error_chain! {
        errors {
            #[doc("The color value supplied was not recognized")]
            InvalidColor {
                description("The color value supplied was not recognized")
                display("The color value supplied was not recognized")
            }
            #[doc("The pattern value supplied was not recognized")]
            InvalidPattern {
                description("The pattern value supplied was not recognized")
                display("The pattern value supplied was not recognized")
            }
            #[doc("The LED number is either invalid or not supported by the connected device")]
            InvalidLED {
                description("The LED number is either invalid or not supported by the connected device")
                display("The LED number is either invalid or not supported by the connected device")
            }
            #[doc("The provided device ID was incorrectly formatted")]
            InvalidDeviceID {
                description("The provided device ID was incorrectly formatted")
                display("The provided device ID was incorrectly formatted")
            }
            #[doc("No device was discovered, or the ID did not resolve to a device")]
            DeviceNotFound {
                description("No device was discovered, or the ID did not resolve to a device")
                display("No device was discovered, or the ID did not resolve to a device")
            }
            #[doc("The server indicated an invalid request")]
            InvalidRequest {
                description("The server indicated an invalid request")
                display("The server indicated an invalid request")
            }
            #[doc("An unexpected HTTP error was returned")]
            UnexpectedError(sc: u16) {
                description("An unexpected HTTP error was returned")
                display("An unexpected HTTP error was returned: {}", sc)
            }
            #[doc("The command is not supported by the current device, or connection to the device")]
            UnsupportedCommand {
                description("The command is not supported by the current device, or connection to the device")
                display("The command is not supported by the current device, or connection to the device")
            }
        }
        foreign_links {
            CustomFmt(::std::num::ParseIntError);
            Request(::reqwest::Error);
            Fmt(::std::fmt::Error);
        }
    }
}

#[cfg(feature = "usb")]
pub mod usb_hid;

#[cfg(feature = "webhook")]
pub mod webhook;
