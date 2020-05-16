use std::net::UdpSocket;
use std::fs;
use std::path::Path;
use ipconfig;

fn main() -> std::io::Result<()> {
    {
        let contents = fs::read("broadcast.bin").expect("broadcast file read failed");
        
        for adapter in ipconfig::get_adapters()? {
            println!("Ip addresses: {:#?}", adapter.ip_addresses());
            let mut socket = UdpSocket::bind("192.168.86.34:34254")?;
            socket.set_broadcast(true).expect("Setting broadcast failed");
            socket.send_to(&contents, "255.255.255.255:67").expect("Couldn't send data");
        }
    } 
    Ok(())
}