#[macro_use]
extern crate log;

use luxafor::{set_pattern, set_solid_color, turn_off, DeviceID, Pattern, SolidColor};
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lux", about = "CLI for Luxafor lights")]
pub(crate) struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub(crate) enum SubCommand {
    /// Set the light to a to a solid color
    Solid {
        /// The device identifier
        #[structopt(long, short)]
        device: DeviceID,

        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,
    },
    /// Set the light to a to a blinking color
    Blink {
        /// The device identifier
        #[structopt(long, short)]
        device: DeviceID,

        /// The color to set
        #[structopt(name = "COLOR")]
        color: SolidColor,
    },
    /// Set the light to a to a pre-defined pattern
    Pattern {
        /// The device identifier
        #[structopt(long, short)]
        device: DeviceID,

        /// The pattern to set
        #[structopt(long, short)]
        pattern: Pattern,
    },
    /// Turn the light off
    Off {
        /// The device identifier
        #[structopt(long, short)]
        device: DeviceID,
    },
}

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

    match args.cmd {
        SubCommand::Solid { device, color } => set_solid_color(device, color, false),
        SubCommand::Blink { device, color } => set_solid_color(device, color, true),
        SubCommand::Pattern { device, pattern } => set_pattern(device, pattern),
        SubCommand::Off { device } => turn_off(device),
    }?;

    Ok(())
}
