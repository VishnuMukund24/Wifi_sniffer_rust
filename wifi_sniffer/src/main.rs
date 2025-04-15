use pcap::{Device, Capture};

fn main() {
    // Choose the interface: eth0 (modify if your interface name is different)
    let interface_name = "eth0";
    
    // Find the desired interface
    let device = Device::list()
        .expect("Failed to list devices")
        .into_iter()
        .find(|d| d.name == interface_name)
        .expect("eth0 interface not found");

    // Open the device in capture mode
    let mut cap = Capture::from_device(device)
        .expect("Failed to create capture")
        .promisc(true)
        .snaplen(65535)
        .timeout(500)
        .open()
        .expect("Failed to open capture on eth0");

    println!("Listening for packets on interface: {}", interface_name);

    // Capture 10 packets
    for i in 1..=10 {
        match cap.next_packet() {
                Ok(packet) => {
                        println!("Packet #{} | Length : {}", i, packet.header.len);
                        println!("Raw Bytes: {:02X?}", &packet.data[..std::cmp::min(32, packet.data.len())]);
                }
                Err(e) => {
                        eprintln!("Error capturing packet: {}", e);
                }
        }
    }
}