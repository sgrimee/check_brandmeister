# check_brandmeister

Simple plugin for [nagios]-compatible monitoring systems to check ham-radio repeater status
on the [BrandMeister] network.

It verifies the last time a ham-radio repeater was seen on the [BrandMeister] network
using [BrandMeister]'s API and compares the number of seconds elapsed to thresholds
for Warning or Critical state. Tested with [LibreNMS].

[![Crates.io](https://img.shields.io/crates/v/check_brandmeister.svg)](https://crates.io/crates/check_brandmeister)
[![GitHub release](https://img.shields.io/github/v/release/sgrimee/check_brandmeister.svg)](https://github.com/sgrimee/check_brandmeister/releases)
[![Crates.io](https://img.shields.io/crates/l/check_brandmeister.svg)](https://raw.githubusercontent.com/sgrimee/check_brandmeister/master/LICENSE)
[![Build Status](https://github.com/sgrimee/check_brandmeister/workflows/CI/badge.svg?branch=master)](https://github.com/sgrimee/check_brandmeister/actions?query=branch%3Amaster)

### Installation
Build the executable and install it in your nagios plugin folder.

Example:
```
cargo install check_brandmeister
sudo mv $HOME/.cargo/bin /usr/lib/nagios/plugins/
```

If you do not want to compile, you may find pre-built binaries on the [releases page](https://github.com/sgrimee/check_brandmeister/releases)

### Usage

The check_brandmeister plugin is called by Nagios or LibreNMS but can be tested on the command-line.

Example:
```
check_brandmeister --repeater 270107 --critical 900

BrandMeister repeater 270107 is OK: online status| 'last_seen'=152s;;900;0;
```

```
USAGE:
    check_brandmeister [OPTIONS] --repeater <repeater>

OPTIONS:
    -c, --critical <seconds>
            Optional: Inactive time in seconds before Critical state

    -h, --help
            Print help information

    -H, --host <host>
            Optional and ignored. For compatibility with nagios Host

    -r, --repeater <id>
            BM repeater id, e.g. 270107

    -V, --version
            Print version information

    -w, --warning <seconds>
            Optional: Inactive time in seconds before Warning state
```

[BrandMeister]: https://brandmeister.network/
[nagios]: https://nagios-plugins.org/doc/guidelines.html
[LibreNMS]: https://www.librenms.org/

License: MIT
