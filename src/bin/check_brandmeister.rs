//! Simple plugin for [nagios]-compatible monitoring systems to check ham-radio repeater status
//! on the [BrandMeister] network.
//!
//! It verifies the last time a ham-radio repeater was seen on the [BrandMeister] network
//! using [BrandMeister]'s API and compares the number of seconds elapsed to thresholds
//! for Warning or Critical state. Tested with [LibreNMS].
//!
//! ## Installation
//! Build the executable and install it in your nagios plugin folder.
//!
//! Example:
//! ```text
//! cargo install check_brandmeister
//! sudo mv $HOME/.cargo/bin /usr/lib/nagios/plugins/
//! ```
//!
//! If you do not want to compile, you may find pre-built binaries on the [releases page](https://github.com/sgrimee/check_brandmeister/releases)
//!
//! ## Usage
//!
//! The check_brandmeister plugin is called by Nagios or LibreNMS but can be tested on the command-line.
//!
//! Example:
//! ```text
//! check_brandmeister --repeater 270107 --critical 900
//!
//! BrandMeister repeater 270107 is OK: online status| 'last_seen'=152s;;900;0;
//! ```
//!
//! ```text
//! USAGE:
//!     check_brandmeister [OPTIONS] --repeater <repeater>
//!
//! OPTIONS:
//!     -c, --critical <seconds>
//!             Inactive time in seconds before Critical state [default: 900]
//!
//!     -h, --help
//!             Print help information
//!
//!     -H, --host <hostname>
//!             Ignored. For compatibility with nagios Host
//!
//!     -r, --repeater <id>
//!             BM repeater id, e.g. 270107
//!
//!     -V, --version
//!             Print version information
//!
//!     -w, --warning <seconds>
//!             Inactive time in seconds before Warning state [default: 600]
//! ```
//!
//! [BrandMeister]: https://brandmeister.network/
//! [nagios]: https://nagios-plugins.org/doc/guidelines.html
//! [LibreNMS]: https://www.librenms.org/

#![warn(missing_docs)]

// use anyhow::Result;
use clap::Parser;
use nagiosplugin::{Metric, Resource, Runner, ServiceState, TriggerIfValue, Unit};

use brandmeister::last_seen_seconds;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ID of the BrandMeister repeater to check
    #[arg(short, long)]
    repeater: u32,
    /// Threshold in seconds for warning state
    #[arg(short, long, default_value_t = 600)]
    warning: i64,
    /// Threshold in seconds for critical state
    #[arg(short, long, default_value_t = 900)]
    critical: i64,
    /// Ignored, for compatibility with nagios host
    #[arg(short = 'H', long)]
    host: Option<String>,
}

fn do_check() -> anyhow::Result<Resource, anyhow::Error> {
    let args = Args::parse();

    let seconds = last_seen_seconds(args.repeater)?;
    let resource = Resource::new(format!("BrandMeister repeater {}", args.repeater))
        .with_description("online status")
        .with_result(
            Metric::new("last_seen", seconds)
                .with_minimum(0)
                .with_unit(Unit::Seconds)
                .with_thresholds(args.warning, args.critical, TriggerIfValue::Greater),
        );
    Ok(resource)
}

fn main() {
    Runner::new()
        .on_error(|e| (ServiceState::Unknown, e))
        .safe_run(do_check)
        .print_and_exit();
}
