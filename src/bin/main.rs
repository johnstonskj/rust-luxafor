#[allow(unused_imports)]
#[macro_use]
extern crate log;

use luxafor::usb_hid::USBDeviceDiscovery;
use luxafor::{webhook, Device, Pattern, SolidColor};
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lux", about = "CLI for Luxafor lights")]
pub(crate) struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    /// The device identifier
    #[structopt(long, short, env = "LUX_DEVICE")]
    device: String,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub(crate) enum SubCommand {
    /// Set the light to a to a solid color
    Solid {
        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,
    },
    /// Set the light to a to a blinking color
    Blink {
        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,
    },
    /// Set the light to a to a pre-defined pattern
    Pattern {
        /// The pattern to set
        pattern: Pattern,
    },
    /// Turn the light off
    Off,
}

const DEVICE_CONNECTION_USB: &str = "usb";

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLine::from_args();

    pretty_env_logger::formatted_builder()
        .filter_level(match args.verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    if args.device == DEVICE_CONNECTION_USB {
        let discovery = USBDeviceDiscovery::new()?;
        let device = discovery.device()?;
        debug!("USB device: '{}'", device.id());
        set_lights(args, device)
    } else {
        let device_id = args.device.clone();
        set_lights(args, webhook::new_device_for(&device_id)?)
    }
}

fn set_lights(args: CommandLine, device: impl Device) -> Result<(), Box<dyn Error>> {
    match args.cmd {
        SubCommand::Solid { color } => device.set_solid_color(color, false),
        SubCommand::Blink { color } => device.set_solid_color(color, true),
        SubCommand::Pattern { pattern } => device.set_pattern(pattern),
        SubCommand::Off => device.turn_off(),
    }?;

    Ok(())
}
