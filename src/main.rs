use std::net::UdpSocket;
use std::time::Duration;
use std::fs;
use ipconfig;

fn main() -> std::io::Result<()> {
    let ip_addresses = get_ip_adresses();
    println!("ip_addresses var: {:?}", ip_addresses);
    for ip_address in ip_addresses {
        
        send_dhcp_broadcast(&ip_address);
        listen_for_dhcp_broadcasts(&ip_address);
        
    }
    Ok(())
}

fn listen_for_dhcp_broadcasts(given_ip: &String){
    let socket = UdpSocket::bind("0.0.0.0:68").unwrap();
    let five_seconds = Duration::new(5,0);
    let mut buf = [0; 100000];
    socket.set_read_timeout(Some(five_seconds));
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    let buf = &mut buf[..amt];
    println!("{}: {:?}", &given_ip, buf);
}

fn send_dhcp_broadcast(given_ip: &String){
    let dhcp_file = fs::read("broadcast.bin").expect("broadcast file read failed");
    let socket = UdpSocket::bind(format!("{}:34254", &given_ip)).unwrap();
    socket.set_broadcast(true).expect("Setting broadcast failed");
    socket.send_to(&dhcp_file, "255.255.255.255:67").expect("Couldn't send data");
}

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