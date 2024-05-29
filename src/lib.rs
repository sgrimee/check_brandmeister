//! This simple library verifies the last time a ham-radio repeater was seen on the [BrandMeister] network
//! using [BrandMeister]'s API v2 returns the time elapsed in seconds.
//! It is not a full client for the brandmeister API.
//!
//! See check_brandmeister for a client implementing a [nagios] plugin using this library.
//!
//! [BrandMeister]: https://brandmeister.network/
//! [nagios]: https://nagios-plugins.org/doc/guidelines.html

#![warn(missing_docs)]

use anyhow::{Context, Result};
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RepeaterStatus {
    last_seen: String,
}

fn get_bm_repeater_last_update(repeater_id: u32) -> Result<String, anyhow::Error> {
    let request_url = format!("https://api.brandmeister.network/v2/device/{}", repeater_id);
    let status: RepeaterStatus = ureq::get(&request_url)
        .call()?
        .into_json()
        .context("error parsing brandmeister API result, ensure repeater id is valid")?;
    Ok(status.last_seen)
}

/// Return the number of seconds since the repeater was seen online on BrandMeister.
///
/// Example:
/// ```no_run
/// use brandmeister::last_seen_seconds;
/// let seconds :i64 = last_seen_seconds(270107).unwrap();
/// ```
pub fn last_seen_seconds(repeater_id: u32) -> Result<i64> {
    let last_update_str = get_bm_repeater_last_update(repeater_id)?;
    let naive_last_update = NaiveDateTime::parse_from_str(&last_update_str, "%Y-%m-%d %H:%M:%S")?;
    let last_update = Utc.from_utc_datetime(&naive_last_update);
    Ok(Utc::now().signed_duration_since(last_update).num_seconds())
}
