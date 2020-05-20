use std::net::{IpAddr, UdpSocket};
use std::time::{Duration, Instant};
use std::fs;
use ipconfig;

fn main() -> std::io::Result<()> {
    let system_ip_addresses = get_system_ip_adresses();
    let mut found_dhcp_server_ips: Vec<IpAddr> = Vec::new();

    for ip_address in &system_ip_addresses {
        send_dhcp_broadcast(&ip_address.to_string());
        for found_ip in listen_for_dhcp_broadcasts() {
            found_dhcp_server_ips.push(found_ip);
        }
    }

    let mut rogue_dhcp_ips: Vec<IpAddr> = Vec::new();
    for found_dhcp_server_ip in found_dhcp_server_ips {
        let mut is_good = false;
        for system_ip_address in &system_ip_addresses {
            if &found_dhcp_server_ip == system_ip_address {
                is_good = true;
            }
        }
        if is_good == false {
            rogue_dhcp_ips.push(found_dhcp_server_ip);
        }
    }

    if rogue_dhcp_ips.len() > 0 {
        println!("Found rogue DHCP servers: {:?}", rogue_dhcp_ips);
    } else {
        println!("0");
    }

    Ok(())
}

fn listen_for_dhcp_broadcasts() -> Vec<IpAddr>{
    let now = Instant::now();
    let five_seconds = Duration::new(5,0);

    let socket = UdpSocket::bind("0.0.0.0:68").unwrap();
    socket.set_read_timeout(Some(five_seconds)).expect("Setting read timeout failed for some reason");
    let mut buf = [0; 10000];
    let mut received_ips: Vec<IpAddr> = Vec::new();
    // let mut ip_addr_string_vec: Vec<String> = Vec::new();
    while now.elapsed() < five_seconds {
        let f = socket.recv_from(&mut buf);
        let f = match f {
            Ok((amt, src)) => Some((amt, src)),
            Err(_) => None,
        };
        if f == None {
            continue
        }
        let m = f.as_ref().unwrap();
        received_ips.push(m.1.ip());
    }

    return received_ips
}

fn send_dhcp_broadcast(_given_ip: &String){
    let dhcp_file = fs::read("broadcast.bin").expect("broadcast file read failed");
    let socket = UdpSocket::bind(format!("{}:34254", &_given_ip)).unwrap();
    socket.set_broadcast(true).expect("Setting broadcast failed");
    socket.send_to(&dhcp_file, "255.255.255.255:67").expect("Couldn't send data");
}

fn get_system_ip_adresses() -> Vec<IpAddr>{
    let adapters = ipconfig::get_adapters();
    let mut ip_addr_string_vec: Vec<IpAddr> = Vec::new();
    for adapter in adapters {
        for m in adapter {
            for ip in m.ip_addresses() {
                if ip.is_ipv4() == true {
                    ip_addr_string_vec.push(*ip);
                }
            }
        }
    }
    ip_addr_string_vec
}