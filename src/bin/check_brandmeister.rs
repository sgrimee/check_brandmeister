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

use anyhow::Result;
use clap::{app_from_crate, arg};
use nagiosplugin::{Metric, Resource, Runner, ServiceState, TriggerIfValue, Unit};

use brandmeister::last_seen_seconds;

#[derive(Debug)]
struct Config {
    repeater_id: u32,
    warn_seconds: Option<i64>,
    critical_seconds: Option<i64>,
}

fn get_config() -> Result<Config> {
    let matches = app_from_crate!()
        .arg(arg!(
            -r --repeater <id> "Sets repeater id to check"
        ))
        .arg(
            arg!(
                -w --warning <seconds> "Threshold for warning state"
            )
            .default_value("600")
            .validator(|s| s.parse::<u32>())
            .required(false),
        )
        .arg(
            arg!(
                -c --critical <seconds> "Threshold for critical state"
            )
            .default_value("900")
            .validator(|s| s.parse::<u32>())
            .required(false),
        )
        .arg(
            arg!(
                -H --host <hostname> "Ignored, for compatibility with nagios Host"
            )
            .required(false),
        )
        .get_matches();

    Ok(Config {
        repeater_id: matches.value_of_t("repeater").expect("required"),
        warn_seconds: matches.value_of_t("warning").ok(),
        critical_seconds: matches.value_of_t("critical").ok(),
    })
}

fn do_check() -> anyhow::Result<Resource, anyhow::Error> {
    let config = get_config()?;
    println!("Config: {:?}", config);
    let seconds = last_seen_seconds(config.repeater_id)?;
    let resource = Resource::new(format!("BrandMeister repeater {}", config.repeater_id))
        .with_description("online status")
        .with_result(
            Metric::new("last_seen", seconds)
                .with_minimum(0)
                .with_unit(Unit::Seconds)
                .with_thresholds(
                    config.warn_seconds,
                    config.critical_seconds,
                    TriggerIfValue::Greater,
                ),
        );
    Ok(resource)
}

fn main() {
    Runner::new()
        .on_error(|e| (ServiceState::Unknown, e))
        .safe_run(do_check)
        .print_and_exit();
}
