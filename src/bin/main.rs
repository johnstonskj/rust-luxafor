#[allow(unused_imports)]
#[macro_use]
extern crate log;

use luxafor::usb_hid::USBDeviceDiscovery;
use luxafor::{webhook, Device, Pattern, SolidColor, Wave};
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
    /// Set the light to a to a strobing/blinking color
    Strobe {
        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,

        /// The speed of each strobe cycle
        #[structopt(long, short, default_value = "10")]
        speed: u8,

        /// The number of times to repeat the strobe
        #[structopt(long, short, default_value = "255")]
        repeat: u8,
    },
    /// Set the light to fade from the current to a new color
    Fade {
        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,

        /// The speed of each strobe cycle
        #[structopt(long, short, default_value = "60")]
        fade_duration: u8,
    },
    /// Set the light to a to a pre-defined wave pattern
    Wave {
        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,

        /// The pattern to set
        #[structopt(default_value = "short")]
        pattern: Wave,

        /// The speed of each wave cycle
        #[structopt(long, short, default_value = "30")]
        speed: u8,

        /// The number of times to repeat the pattern
        #[structopt(long, short, default_value = "255")]
        repeat: u8,
    },
    /// Set the light to a to a pre-defined pattern
    Pattern {
        /// The pattern to set
        pattern: Pattern,

        /// The number of times to repeat the pattern
        #[structopt(long, short, default_value = "255")]
        repeat: u8,
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
        SubCommand::Solid { color } => device.set_solid_color(color),
        SubCommand::Fade {
            color,
            fade_duration,
        } => device.set_fade_to_color(color, fade_duration),
        SubCommand::Strobe {
            color,
            speed,
            repeat,
        } => device.set_color_strobe(color, speed, repeat),
        SubCommand::Wave {
            color,
            pattern,
            speed,
            repeat,
        } => device.set_color_wave(color, pattern, speed, repeat),
        SubCommand::Pattern { pattern, repeat } => device.set_pattern(pattern, repeat),
        SubCommand::Off => device.turn_off(),
    }?;

    Ok(())
}
