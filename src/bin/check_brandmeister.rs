// #![warn(missing_docs)]

use anyhow::{anyhow, Context, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use nagiosplugin::{Metric, Resource, Runner, ServiceState, TriggerIfValue};

use check_brandmeister::last_seen_minutes;

struct Config {
    repeater_id: u32,
    warn_minutes: i64,
    critical_minutes: i64,
}

fn get_config() -> Result<Config> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("repeater")
                .short('r')
                .long("repeater")
                .value_name("repeater")
                .required(true)
                .about("BM repeater id, e.g. 270107")
                .takes_value(true),
        )
        .arg(
            Arg::new("warn_minutes")
                .short('w')
                .long("warn")
                .value_name("warn_minutes")
                .required(false)
                .about("Inactive time in minutes before Warning state")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::new("critical_minutes")
                .short('c')
                .long("critical")
                .value_name("critical_minutes")
                .required(false)
                .about("Inactive time in minutes before Critical state")
                .takes_value(true)
                .default_value("15"),
        )
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_name("host")
                .required(false)
                .about("Ignored. For compatibility with nagios Host")
                .takes_value(true),
        )
        .get_matches();

    let repeater_id = matches
        .value_of_t::<u32>("repeater")
        .context("could not convert given repeater id to integer")?;
    let mut warn_minutes = matches
        .value_of_t::<i64>("warn_minutes")
        .context("could not convert warn_minutes to integer")?;
    let critical_minutes = matches
        .value_of_t::<i64>("critical_minutes")
        .context("could not convert critical_minutes to integer")?;
    if warn_minutes > critical_minutes {
        warn_minutes = critical_minutes
    };
    Ok(Config {
        repeater_id,
        warn_minutes,
        critical_minutes,
    })
}

fn do_check() -> anyhow::Result<Resource> {
    let config = get_config()?;
    let minutes = last_seen_minutes(config.repeater_id)?;
    let resource = Resource::new(format!("BrandMeister repeater {}", config.repeater_id))
        .with_description("online status")
        // .with_result(Metric::new("last_seen_min", minutes).with_maximum(15)); // this is buggy, does not trigger
        .with_result(Metric::new("last_seen_min", minutes).with_thresholds(
            config.warn_minutes,
            config.critical_minutes,
            TriggerIfValue::Greater,
        ));
    Ok(resource)
}

fn main() {
    Runner::new()
        .on_error(|e| -> (ServiceState, anyhow::Error) {
            (ServiceState::Unknown, anyhow!(e.to_string()))
        })
        .safe_run(do_check)
        .print_and_exit();
}
