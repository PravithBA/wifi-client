# Wifi Client

`wifi-client` is a tool for connecting to a wifi easily.

This tool uses `nmcli` under the hood.

## Installation

    cargo install wifi-client

## Usage

> **_NOTE:_**  You have to have have `nmcli` installed for this tool to work.
Look at nmcli [man page](https://www.linux.org/docs/man1/nmcli.html) for installation.

Command:

    wifi-client

Output:

    ▰▰▰▱▱▱▱ Scanning for wifi
    ? Select the SSID you want to connect
      > SSID: "wifi 1"
        SSID: "wifi 2"
    [↑↓ to move, enter to select, type to filter]

After selection:

    ▰▰▰▱▱▱▱ Scanning for wifi
	> Select the SSID you want to connect SSID: "wifi 1"
	? Please enter the password for "SSID: "wifi 1""

After password input:

    ▰▰▰▱▱▱▱ Scanning for wifi
	> Select the SSID you want to connect SSID: "wifi 1"
	> Please enter the password for "SSID: "wifi 1"" ********
	Device '<network-interface-name>' successfully activated with 'XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX'.

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am "Add some feature"`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request

