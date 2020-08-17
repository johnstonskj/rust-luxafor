#Crate luxafor

Library, and CLI, for [Luxafor](https://luxafor.com/products/) lights via webhooks.

![Rust](https://github.com/johnstonskj/luxafor/workflows/Rust/badge.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/atelier_core.svg)](https://crates.io/crates/atelier_core)
[![docs.rs](https://docs.rs/atelier_core/badge.svg)](https://docs.rs/atelier_core)
![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/luxafor.svg)](https://github.com/johnstonskj/luxafor/stargazers)

This has been tested with the USB connected [flag](https://luxafor.com/flag-usb-busylight-availability-indicator/)
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


## Changes

**Version 0.1.0**

* Initial commit, supports flag and bluetooth lights.


## TODO

TBD