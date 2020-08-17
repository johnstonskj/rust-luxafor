# Crate luxafor

Library, and CLI, for [Luxafor](https://luxafor.com/products/) lights via webhooks.

![Rust](https://github.com/johnstonskj/rust-luxafor/workflows/Rust/badge.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/luxafor.svg)](https://crates.io/crates/luxafor)
[![docs.rs](https://docs.rs/luxafor/badge.svg)](https://docs.rs/luxafor)
![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-luxafor.svg)](https://github.com/johnstonskj/rust-luxafor/stargazers)

This has been tested with the USB connected [flag](https://luxafor.com/flag-usb-busylight-availability-indicator/)
as well as the [Bluetooth](https://luxafor.com/bluetooth-busy-light-availability-indicator/) lights.

# Examples

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


## Changes

**Version 0.1.0**

* Initial commit, supports flag and bluetooth lights.


## TODO

TBD