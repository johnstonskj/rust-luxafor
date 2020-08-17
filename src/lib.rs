/*!
Library, and CLI, for [Luxafor](https://luxafor.com/products/) lights via webhooks. This has been
tested with the USB connected [flag](https://luxafor.com/flag-usb-busylight-availability-indicator/)
as well as the [Bluetooth](https://luxafor.com/bluetooth-busy-light-availability-indicator/) lights.

# Examples

The following shows the command line tool setting the color to red.

```bash
❯ lux solid red -d 2a0f2c73b72
```

The following shows the command line tool setting the color to a blinking green.

```bash
❯ lux blink green -d 2a0f2c73b72
```

The following shows the command line tool turning the light off.

```bash
❯ lux -vvv off -d 2a0f2c73b72
 INFO  luxafor > Setting the color of device '2a0f2c73b72e' to 000000
 INFO  luxafor > call successful
```

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
    //missing_docs,
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

#[macro_use]
extern crate log;

use reqwest::blocking::Client;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This wraps a simple string and ensures it only contains valid characters.
///
#[derive(Clone, Debug)]
pub struct DeviceID(String);

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
    Custom { red: u8, green: u8, blue: u8 },
}

///
/// A pattern the light can be set to show.
#[derive(Clone, Debug)]
pub enum Pattern {
    /// A preset pattern
    Police,
    /// A preset pattern
    TrafficLights,
    /// A preset pattern
    Random(u8),
    /// A preset pattern (accepted on Windows only)
    Rainbow,
    /// A preset pattern (accepted on Windows only)
    Sea,
    /// A preset pattern (accepted on Windows only)
    WhiteWave,
    /// A preset pattern (accepted on Windows only)
    Synthetic,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const API_V1: &str = "https://api.luxafor.com/webhook/v1/actions";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Turn the light off.
///
pub fn turn_off(device: DeviceID) -> error::Result<()> {
    set_solid_color(
        device,
        SolidColor::Custom {
            red: 00,
            green: 00,
            blue: 00,
        },
        false,
    )
}

///
/// Set the color, and blink status,  of the light.
///
pub fn set_solid_color(device: DeviceID, color: SolidColor, blink: bool) -> error::Result<()> {
    info!("Setting the color of device '{}' to {}", device, color);

    let body = if let SolidColor::Custom {
        red: _,
        green: _,
        blue: _,
    } = color
    {
        r#"{
  "userId": "DID",
  "actionFields":{
    "color": "custom",
    "custom_color": "COLOR"
  }
}"#
        .replace("DID", &device.to_string())
        .replace("COLOR", &color.to_string())
    } else {
        r#"{
  "userId": "DID",
  "actionFields":{
    "color": "COLOR"
  }
}"#
        .replace("DID", &device.to_string())
        .replace("COLOR", &color.to_string())
    };

    let url = &format!("{}/{}", API_V1, if blink { "blink" } else { "solid_color" });

    send_request(url, body)
}

///
/// Set the pattern displayed by the light.
///
pub fn set_pattern(device: DeviceID, pattern: Pattern) -> error::Result<()> {
    info!("Setting the pattern of device '{}' to {}", device, pattern);

    let body = r#"{
  "userId": "DID",
  "actionFields":{
    "pattern": "PATTERN"
  }
}"#
    .replace("DID", &device.to_string())
    .replace("PATTERN", &pattern.to_string());

    let url = &format!("{}/{}", API_V1, "pattern");

    send_request(url, body)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for DeviceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DeviceID {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(s.to_string()))
        } else {
            Err(error::ErrorKind::InvalidDeviceID.into())
        }
    }
}

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

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pattern::Police => "police".to_string(),
                Pattern::TrafficLights => "traffic lights".to_string(),
                Pattern::Random(n) => format!("random {}", n),
                Pattern::Rainbow => "rainbow".to_string(),
                Pattern::Sea => "sea".to_string(),
                Pattern::WhiteWave => "white wave".to_string(),
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
            "sea" => Ok(Pattern::Sea),
            "white wave" => Ok(Pattern::WhiteWave),
            "synthetic" => Ok(Pattern::Synthetic),
            _ => Err(error::ErrorKind::InvalidPattern.into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn send_request(api: &str, body: String) -> error::Result<()> {
    debug!("Sending to: {}", api);
    debug!("Sending data: {:?}", body);

    let client = Client::new();
    let result = client
        .post(api)
        .header("Content-Type", "application/json")
        .body(body)
        .send()?;

    if result.status().is_success() {
        info!("call successful");
        Ok(())
    } else {
        let status_code = result.status().as_u16();
        error!("call failed");
        error!("{:?}", result.text());
        Err(error::ErrorKind::UnexpectedError(status_code).into())
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod error {
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
            #[doc("The provided device ID was incorrectly formatted")]
            InvalidDeviceID {
                description("The provided device ID was incorrectly formatted")
                display("The provided device ID was incorrectly formatted")
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
        }
        foreign_links {
            CustomFmt(::std::num::ParseIntError);
            Request(::reqwest::Error);
            Fmt(::std::fmt::Error);
        }
    }
}
