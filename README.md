# Rust Packet Sniffer

This program is a simple packet sniffer implemented in Rust using the `pnet` crate. It captures and processes network packets on a specified interface.

# Rust Packet Sniffer

This program is a simple packet sniffer implemented in Rust using the `pnet` crate. It captures and processes network packets on a specified interface.

## How It Works

The program listens on a specified network interface for incoming packets. When a packet is captured:

- If the packet is an Ethernet packet, its raw data is printed.
- If the packet is also an IPv4 packet, the program further inspects for a TCP segment.
- Upon identifying a TCP segment within the IPv4 packet, it parses and prints the TCP header along with source and destination information.

## Usage

Compile the program and run it as follows, substituting `<interface_name>` with the name of the interface you want to listen to:

```sh
cargo run <interface_name>
```

You will need administrative privileges to capture packets; on Unix systems, you may need to use `sudo`:

```sh
sudo ./target/debug/pnet-example <interface_name>
```

Note: `<compiled_binary>` is the name of the compiled program that Cargo creates.

## Dependencies

Ensure that the (pnet)[https://crates.io/crates/pnet] crate is included in your `Cargo.toml`:

```toml
[dependencies]
pnet = "0.34.0"
```

### Windows

There are three requirements for building on Windows:

You must use a version of Rust which uses the MSVC toolchain
You must have WinPcap or npcap installed (tested with version WinPcap 4.1.3) (If using npcap, make sure to install with the "Install Npcap in WinPcap API-compatible Mode")
You must place Packet.lib from the WinPcap Developers pack in a directory named lib, in the root of this repository. Alternatively, you can use any of the locations listed in the %LIB%/$Env:LIB environment variables. For the 64 bit toolchain it is in WpdPack/Lib/x64/Packet.lib, for the 32 bit toolchain, it is in WpdPack/Lib/Packet.lib.

## Disclaimer

This software is for educational purposes only. Ensure you have permission to capture traffic on the network you are monitoring and that you comply with all relevant laws and regulations.

## Acknowledgments

This packet sniffer is based on an example from the book _"Network Programming with Rust"_ by Abhishek Chanda. Credit goes to the author for the original concept and implementation. This README summarizes the program and its workings.
