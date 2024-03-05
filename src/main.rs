// pnet crate example, create a simple packet sniffer and dumper

extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    Packet,
};
use std::env;

// Handles a single ethernet packet
fn handle_packet(ethernet: &EthernetPacket) {
    // Print the entire Ethernet frame
    // println!("Got an ethernet packet: {:?}", ethernet);
    // Determine the type of packet encapsulated within the Ethernet frame
    match ethernet.get_ethertype() {
        // If it's an IPv4 packet, handle it
        EtherTypes::Ipv4 => {
            // println!("IPv4 packet");
            // Attempt to construct an IPv4 packet from the Ethernet payload
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                // Print the IPv4 header
                // println!("Header: {:?}", header);
                // Check the protocol of the encapsulated data
                match header.get_next_level_protocol() {
                    // If it's a TCP packet, handle it
                    IpNextHeaderProtocols::Tcp => {
                        // println!("TCP packet");
                        // Attempt to construct a TCP packet from the IPv4 payload
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            // Print the TCP packet
                            // println!("TCP packet: {:?}", tcp);
                            // Print source and destination IPs and ports
                            println!(
                                "Got a TCP packet from {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        } else {
                            // The TCP packet couldn't be parsed
                            println!("Malformed TCP packet");
                        }
                    }
                    // Ignore packets that are not TCP
                    _ => {
                        println!("Ignoring non-TCP packet");
                    }
                }
            } else {
                // The IPv4 packet couldn't be parsed
                println!("Malformed IPv4 packet");
            }
        }
        // Ignore packets that are not IPv4
        _ => {
            println!("Ignoring non-IPv4 packet");
        }
    }
}

fn main() {
    // Print a starting message
    println!("Starting the packet sniffer...\n");

    // Get the network interface name from the command line arguments
    let interface_name = env::args().nth(1).expect("No interface name provided\n");
    // Print the network command line argument
    println!("Selected interface: {}\n", interface_name);
    // Retrieve the list of available network interfaces
    let interfaces = datalink::interfaces();
    // Print the list of network interfaces
    // println!("Available interfaces...");
    // for iface in &interfaces {
    //     println!(
    //         "Interface: {}, IP: {:?}",
    //         iface.name,
    //         iface.ips.iter().map(|ip| ip.ip()).collect::<Vec<_>>()
    //     );
    // }
    // Find the specified network interface by name
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("\nError getting interface\n");

    // Print the network interface
    println!("\nAccepted interface: {:?}\n", interface);
    // Create a channel to receive packets from the network interface
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type\n"),
        Err(e) => panic!("Error creating channel: {}\n", e),
    };

    // Enter an infinite loop to process packets as they arrive
    loop {
        match rx.next() {
            // When a packet is received, handle it
            Ok(packet) => {
                // Construct an Ethernet packet from the raw data
                let packet = EthernetPacket::new(packet).unwrap();
                // Pass the Ethernet packet to the handler function
                handle_packet(&packet);
            }
            // If there's an error receiving a packet, print an error message
            Err(e) => {
                println!("Error receiving packet: {}\n", e);
            }
        }
    }
}
