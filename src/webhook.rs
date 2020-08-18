/*!
Implementation of the Device trait for webhook connected lights.

*/

use crate::{Device, DeviceIdentifier, Pattern, SolidColor};
use reqwest::blocking::Client;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The device identifier for a webhook connected light.
///
#[derive(Clone, Debug)]
pub struct WebhookDeviceID(String);

///
/// The device implementation for a webhook connected light.
///
#[derive(Clone, Debug)]
pub struct WebhookDevice {
    id: WebhookDeviceID,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return a device implementation for a webhook connected light.
///
pub fn new_device_for(device_id: &str) -> crate::error::Result<impl Device> {
    let device_id = WebhookDeviceID::from_str(device_id)?;
    Ok(WebhookDevice { id: device_id })
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

const API_V1: &str = "https://api.luxafor.com/webhook/v1/actions";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for WebhookDeviceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DeviceIdentifier for WebhookDeviceID {}

impl FromStr for WebhookDeviceID {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(s.to_string()))
        } else {
            Err(crate::error::ErrorKind::InvalidDeviceID.into())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Device for WebhookDevice {
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
            .replace("DID", &self.id.to_string())
            .replace("COLOR", &color.to_string())
        } else {
            r#"{
  "userId": "DID",
  "actionFields":{
    "color": "COLOR"
  }
}"#
            .replace("DID", &self.id.to_string())
            .replace("COLOR", &color.to_string())
        };

        let url = &format!("{}/{}", API_V1, if blink { "blink" } else { "solid_color" });

        send_request(url, body)
    }

    fn set_pattern(&self, pattern: Pattern) -> crate::error::Result<()> {
        info!("Setting the pattern of device '{}' to {}", self.id, pattern);

        let body = r#"{
  "userId": "DID",
  "actionFields":{
    "pattern": "PATTERN"
  }
}"#
        .replace("DID", &self.id.to_string())
        .replace("PATTERN", &pattern.to_string());

        let url = &format!("{}/{}", API_V1, "pattern");

        send_request(url, body)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn send_request(api: &str, body: String) -> crate::error::Result<()> {
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
        Err(crate::error::ErrorKind::UnexpectedError(status_code).into())
    }
}