#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use chrono::Duration;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use nagiosplugin::{Resource, Runner, ServiceState};
use serde::Deserialize;
use serde_json;
use std::{collections::HashMap, io::Repeat};

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
        .get_matches();
    matches
        .value_of_t::<u32>("repeater")
        .context("could not convert given repeater id to integer")
}

#[derive(Debug, Deserialize)]
struct RepeaterStatus {
    last_updated: String,
}

fn get_bm_repeater_last_update(repeater_id: u32) -> Result<String, anyhow::Error> {
    let request_url = format!(
        "https://api.brandmeister.network/v1.0/repeater/?action=get&q={}",
        repeater_id
    );
    let status: RepeaterStatus = reqwest::blocking::get(&request_url)?
        .json()
        .context("error parsing API result, ensure repeater id is valid")?;
    Ok(String::from(status.last_updated))
}

fn is_online(repeater_id: u32) -> Result<bool> {
    let last_updated = get_bm_repeater_last_update(repeater_id)?;
    let last_updated = Utc.datetime_from_str(&last_updated, "%Y-%m-%d %H:%M:%S")?;
    let duration = Utc::now().signed_duration_since(last_updated);
    Ok(duration <= Duration::minutes(15))
}

fn do_check() -> anyhow::Result<Resource> {
    let repeater_id = repeater_id_from_cli_args()?;
    let state = match is_online(repeater_id)? {
        true => ServiceState::Ok,
        false => ServiceState::Critical,
    };
    let resource = Resource::new("repeater")
        .with_description("BrandMeister repeater online status")
        .with_fixed_state(state);
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
