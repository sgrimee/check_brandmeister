//! This simple library verifies the last time a ham-radio repeater was seen on the [BrandMeister] network
//! using [BrandMeister]'s API returns the time elapsed in seconds.
//! It is not a full client for the brandmeister API.
//!
//! See check_brandmeister for a client implementing a [nagios] plugin using this library.
//!
//! [BrandMeister]: https://brandmeister.network/
//! [nagios]: https://nagios-plugins.org/doc/guidelines.html

#![warn(missing_docs)]

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
        .call()?
        .into_json()
        .context("error parsing brandmeister API result, ensure repeater id is valid")?;
    Ok(status.last_updated)
}

/// Return the number of seconds since the repeater was seen online on BrandMeister.
///
/// Example:
/// ```no_run
/// use check_brandmeister::last_seen_seconds;
/// let min :u32 = last_seen_seconds("270107");
/// ```
pub fn last_seen_seconds(repeater_id: u32) -> Result<i64> {
    let last_update_str = get_bm_repeater_last_update(repeater_id)?;
    let last_update = Utc.datetime_from_str(&last_update_str, "%Y-%m-%d %H:%M:%S")?;
    Ok(Utc::now().signed_duration_since(last_update).num_seconds())
}
