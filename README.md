# Crate luxafor

Library, and CLI, for [Luxafor](https://luxafor.com/products/) lights via either USB or webhooks.

![Rust](https://github.com/johnstonskj/rust-luxafor/workflows/Rust/badge.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/luxafor.svg)](https://crates.io/crates/luxafor)
[![docs.rs](https://docs.rs/luxafor/badge.svg)](https://docs.rs/luxafor)
![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-luxafor.svg)](https://github.com/johnstonskj/rust-luxafor/stargazers)

The main entry point for clients is the trait `Device` that has implementations for USB connected devices such as the 
[flag](https://luxafor.com/flag-usb-busylight-availability-indicator/) as well as webhooks for both the flag and 
[bluetooth](https://luxafor.com/bluetooth-busy-light-availability-indicator/) lights.

Each connection has its own discovery or connection methods but will provide a `Device` implementation
for the manipulation of the light state.

## API Examples

The following example shows a function that sets the light to a solid red color. It demonstrates
the use of a USB connected device.

```rust
use luxafor::usb_hid::USBDeviceDiscovery;
use luxafor::{Device, SolidColor};
use luxafor::error::Result;

fn set_do_not_disturb() -> Result<()> {
    let discovery = USBDeviceDiscovery::new()?;
    let device = discovery.device()?;
    println!("USB device: '{}'", device.id());
    device.set_solid_color(SolidColor::Red, false)
}
```

The following shows the same function but using the webhook connection. 

```rust
use luxafor::webhook::new_device_for;
use luxafor::{Device, SolidColor};
use luxafor::error::Result;

fn set_do_not_disturb(device_id: &str) -> Result<()> {
    let device = new_device_for(device_id)?;
    println!("Webhook device: '{}'", device.id());
    device.set_solid_color(SolidColor::Red, false)
}
```

## CLI Examples

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

## Features

* **command-line**; provides the command line tool `lux`, it is not on by default for library clients.
* **usb**; provides access to USB connected devices.
* **webhook** (default); provides access to USB, or Bluetooth, devices via webhooks.

## Changes

**Version 0.2.1**

* Removed the `DeviceIdentifier` trait, and `Device` now returns a String.

**Version 0.2.0**

* Refactored to provide a new `Device` trait
* Implemented the trait for webhook connected lights
* Added a new implementation for HID connected lights

**Version 0.1.0**

* Initial commit, supports flag and bluetooth lights.


## TODO

* The webhook API is not as rich as the USB, need to find a way to manage this.