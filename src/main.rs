// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

use anyhow::{anyhow, Context, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use nagiosplugin::{Metric, Resource, Runner, ServiceState, TriggerIfValue};

use check_brandmeister::last_seen_minutes;

fn repeater_id_from_cli_args() -> Result<u32> {
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
            Arg::new("host")
                .short('H')
                .long("host")
                .value_name("host")
                .required(false)
                .about("Host for nagios, but ignored.")
                .takes_value(true),
        )
        .get_matches();
    matches
        .value_of_t::<u32>("repeater")
        .context("could not convert given repeater id to integer")
}

pub fn do_check() -> anyhow::Result<Resource> {
    let repeater_id = repeater_id_from_cli_args()?;
    let minutes = last_seen_minutes(repeater_id)?;
    let resource = Resource::new(format!("BrandMeister repeater {}", repeater_id))
        .with_description("online status")
        // .with_result(Metric::new("last_seen_min", minutes).with_maximum(15)); // this is buggy, does not trigger
        .with_result(Metric::new("last_seen_min", minutes).with_thresholds(
            5,
            15,
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
