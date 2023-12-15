use get_if_addrs::{get_if_addrs, IfAddr};
use std::net::TcpStream;
use std::time::Duration;

pub const MOST_COMMON_PORTS_1002: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111,
    995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
    1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646,
    5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357,
    427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432,
    1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
    3001, 5001, 82, 10010, 1030, 9090, 2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255,
    // ...
];

pub fn scan_common_ports() -> Vec<u16> {
    let mut open_ports = Vec::new();

    if let Ok(if_addrs) = get_if_addrs() {
        let local_ip = if_addrs
            .into_iter()
            .find(|if_addr| match if_addr.addr {
                IfAddr::V4(ref addr) => !addr.ip.is_loopback(),
                IfAddr::V6(_) => false,
            })
            .map(|if_addr| match if_addr.addr {
                IfAddr::V4(addr) => addr.ip,
                IfAddr::V6(_) => unreachable!(),
            });

        if let Some(ip) = local_ip {
            for &port in MOST_COMMON_PORTS_1002 {
                if let Ok(stream) = TcpStream::connect_timeout(&(ip, port).into(), Duration::from_secs(1)) {
                    println!("Port {} is open", port);
                    open_ports.push(port);
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                }
            }
        }
    }

    open_ports
}
