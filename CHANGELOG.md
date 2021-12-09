# Changelog

## [0.2.0] - 2021-12-09

This minor release improves command-line argument parsing.

### New

- warning and critical parameters are both optional
- added Unit (s) and minimum value (0) to the metric

### Changed

#### Breaking

- the metric, now called last_seen is specified in seconds instead of minutes, to comply with the nagios guidelines
- changed CLI parameters long name sto set threshold, now warning and critical

### Fixed

- more meaningful error message when the given repeated id is invalid
