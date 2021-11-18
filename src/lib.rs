//#![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RepeaterStatus {
    last_updated: String,
}

fn get_bm_repeater_last_update(repeater_id: u32) -> Result<String, anyhow::Error> {
    let request_url = format!(
        "http://api.brandmeister.network/v1.0/repeater/?action=get&q={}",
        repeater_id
    );
    let status: RepeaterStatus = ureq::get(&request_url)
        .call()
        .context("error parsing API result, ensure repeater id is valid")?
        .into_json()?;
    Ok(String::from(status.last_updated))
}

pub fn last_seen_minutes(repeater_id: u32) -> Result<i64> {
    let last_update_str = get_bm_repeater_last_update(repeater_id)?;
    let last_update = Utc.datetime_from_str(&last_update_str, "%Y-%m-%d %H:%M:%S")?;
    Ok(Utc::now().signed_duration_since(last_update).num_minutes())
}

// pub fn is_online(repeater_id: u32) -> Result<bool> {
//     let duration = last_seen_duration(repeater_id)?;
//     Ok(duration <= Duration::minutes(15))
// }
