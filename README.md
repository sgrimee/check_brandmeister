# check_brandmeister

Simple plugin for [nagios]-compatible monitoring systems to check ham-radio repeater status
on the [BrandMeister] network.

It verifies the last time a ham-radio repeater was seen on the [BrandMeister] network
using [BrandMeister]'s API and compares the number of minutes elapsed to thresholds
for Warning or Critical state. Tested with [LibreNMS].

### Installation
Build the executable and install it in your nagios plugin folder.

Example:
```
cargo install check_brandmeister
sudo mv $HOME/.cargo/bin /usr/lib/nagios/plugins/
```

### Usage

The check_brandmeister plugin is called by Nagios or LibreNMS but can be tested on the command-line.

Example:
```
check_brandmeister --repeater 270107

BrandMeister repeater 270107 is OK: online status| 'last_seen_min'=0;10;15;;
```

```
USAGE:
    check_brandmeister [OPTIONS] --repeater <repeater>

OPTIONS:
    -c, --critical <critical_minutes>
            Inactive time in minutes before Critical state [default: 15]

    -h, --help
            Print help information

    -H, --host <host>
            Ignored. For compatibility with nagios Host

    -r, --repeater <repeater>
            BM repeater id, e.g. 270107

    -V, --version
            Print version information

    -w, --warn <warn_minutes>
            Inactive time in minutes before Warning state [default: 10]
```

[BrandMeister]: https://brandmeister.network/
[nagios]: https://nagios-plugins.org/doc/guidelines.html
[LibreNMS]: https://www.librenms.org/

License: MIT
