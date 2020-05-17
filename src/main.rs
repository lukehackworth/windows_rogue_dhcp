use std::net::UdpSocket;
use std::time::Duration;
use std::fs;
use ipconfig;

fn main() -> std::io::Result<()> {

    let dhcp_file = fs::read("broadcast.bin").expect("broadcast file read failed");
    let ip_addresses = get_ip_adresses();
    println!("ip_addresses var: {:?}", ip_addresses);
    for ip_address in ip_addresses {
        let socket = UdpSocket::bind(format!("{}:34254", &ip_address))?;
        let socket2 = UdpSocket::bind("0.0.0.0:68")?;

        socket.set_broadcast(true).expect("Setting broadcast failed");
        socket.send_to(&dhcp_file, "255.255.255.255:67").expect("Couldn't send data");

        let five_seconds = Duration::new(5,0);

        let mut buf = [0; 100000];
        socket2.set_read_timeout(Some(five_seconds));
        let (amt, src) = socket2.recv_from(&mut buf)?;
        let buf = &mut buf[..amt];
        println!("{}: {:?}",ip_address, buf);
    }
     
    Ok(())
}
//receive any dhcp offers

// fn get_mac_addresses() -> Vec<String>{

// }

fn get_ip_adresses() -> Vec<String>{
    let adapters = ipconfig::get_adapters();
    let mut ip_addr_string_vec: Vec<String> = Vec::new();
    for adapter in adapters {
        for m in adapter {
            for ip in m.ip_addresses() {
                if ip.is_ipv4() == true {
                    ip_addr_string_vec.push(ip.to_string());
                }
            }
        }
    }
    ip_addr_string_vec
}