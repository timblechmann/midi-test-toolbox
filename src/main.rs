mod analysis;
mod dump;
mod echo;
mod generate;
mod generator;
mod list_devices;
mod loopback_timer;
mod utils;

use clap::{command, Parser, Subcommand};
use inline_colorization::*;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Use real-time scheduling
    #[arg(short, long)]
    rt: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all midi devices
    ListDevices {},

    /// Echo midi input to output port
    Echo {
        #[arg(short, long)]
        /// Input device
        input: String,

        #[arg(short, long)]
        /// Output device
        output: String,

        #[arg(short, long)]
        /// Print message to command line
        print: bool,
    },

    /// Print messages to command line
    Dump {
        #[arg(short, long)]
        input: String,
    },

    /// Generate test notes
    Generate {
        #[arg(long, default_value = "1000")]
        /// Note duration (in milliseconds)
        note_duration: u32,

        /// Notes per second
        #[arg(long, default_value = "2")]
        notes_per_second: u32,

        #[arg(short, long)]
        /// Output device
        output: String,

        #[arg(short, long)]
        /// Print message to command line
        print: bool,

        #[arg(short, long)]
        /// Validate loopback
        loopback_input: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.rt {
        #[cfg(target_os = "linux")]
        utils::acquire_rt_scheduling();
    }

    let result = match &cli.command {
        Some(Commands::ListDevices {}) => list_devices::list_devices(),
        Some(Commands::Echo {
            input,
            output,
            print,
        }) => echo::echo(input, output, *print),
        Some(Commands::Dump { input }) => dump::dump(input),
        Some(Commands::Generate {
            note_duration,
            notes_per_second,
            output,
            print,
            loopback_input,
        }) => match loopback_input {
            None => generate::generate_notes(
                Duration::from_millis((*note_duration).into()),
                Duration::from_secs(1) / *notes_per_second,
                output,
                *print,
                None,
            ),
            Some(input_device) => generate::generate_and_analyse(
                Duration::from_millis((*note_duration).into()),
                Duration::from_secs(1) / *notes_per_second,
                output,
                input_device,
                *print,
            ),
        },
        None => Ok(()),
    };

    if let Err(e) = result {
        eprintln!("{color_red}{style_bold}{}{color_reset}{style_reset}", e);
    }
}
