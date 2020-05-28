use std::net::*;

pub fn global(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(ip) => global_v4(ip),
        IpAddr::V6(ip) => global_v6(ip)
    }
}


pub fn global_v6(addr: &Ipv6Addr) -> bool {
    if addr.is_multicast() {
        match addr.segments()[0] & 0x000f {
            14 => true, // Ipv6MulticastScope::Global
            _ => false,
        }
    } else {
        !addr.is_loopback()
            && !unicast_link_local_v6(addr)
            && !unique_local_v6(addr)
            && !addr.is_unspecified()
            && !documentation_v6(addr)
    }
}

pub fn unicast_link_local_v6(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xffc0) == 0xfe80
}

pub fn unique_local_v6(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xfe00) == 0xfc00
}

pub fn documentation_v6(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] == 0x2001) && (addr.segments()[1] == 0xdb8)
}

pub fn global_v4(addr: &Ipv4Addr) -> bool {
    // check if this address is 192.0.0.9 or 192.0.0.10. These addresses are the only two
    // globally routable addresses in the 192.0.0.0/24 range.
    let u32_form = u32::from(addr.clone());
    if u32_form == 0xc0000009 || u32_form == 0xc000000a {
        return true;
    }

    !addr.is_private()
        && !addr.is_loopback()
        && !addr.is_link_local()
        && !addr.is_broadcast()
        && !addr.is_documentation()
        && !shared_v4(&addr)
        && !ietf_protocol_assignment_v4(&addr)
        && !reserved_v4(&addr)
        && !benchmarking_v4(&addr)
        // Make sure the address is not in 0.0.0.0/8
        && addr.octets()[0] != 0
}

fn benchmarking_v4(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 198 && (addr.octets()[1] & 0xfe) == 18
}

fn reserved_v4(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] & 240 == 240 && !addr.is_broadcast()
}

fn ietf_protocol_assignment_v4(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 192 && addr.octets()[1] == 0 && addr.octets()[2] == 0
}
fn shared_v4(addr: &Ipv4Addr) -> bool {
    addr.octets()[0] == 100 && (addr.octets()[1] & 0b1100_0000 == 0b0100_0000)
}

// Tests for this module
#[cfg(all(test, not(target_os = "emscripten")))]
mod tests {
    //use std::net::test::{sa4, sa6, tsa};
    use std::net::*;
    use std::str::FromStr;

    #[test]
    fn test_custom_global() {
        macro_rules! ip {
            ($s:expr) => {
                &IpAddr::from_str($s).unwrap()
            };
        }

        macro_rules! check {
            ($s:expr) => {
                check!($s, 0);
            };

            ($s:expr, $mask:expr) => {{
                let global: u8 = 1 << 2;
                if ($mask & global) == global {
                    assert!(crate::global(ip!($s)));
                } else {
                    assert!(!crate::global(ip!($s)));
                }
            }};
        }
        let unspec: u8 = 1 << 0;
        let loopback: u8 = 1 << 1;
        let global: u8 = 1 << 2;
        let multicast: u8 = 1 << 3;
        let doc: u8 = 1 << 4;

        check!("0.0.0.0", unspec);
        check!("0.0.0.1");
        check!("0.1.0.0");
        check!("10.9.8.7");
        check!("127.1.2.3", loopback);
        check!("172.31.254.253");
        check!("169.254.253.242");
        check!("192.0.2.183", doc);
        check!("192.1.2.183", global);
        check!("192.168.254.253");
        check!("198.51.100.0", doc);
        check!("203.0.113.0", doc);
        check!("203.2.113.0", global);
        check!("224.0.0.0", global | multicast);
        check!("239.255.255.255", global | multicast);
        check!("255.255.255.255");
        // make sure benchmarking addresses are not global
        check!("198.18.0.0");
        check!("198.18.54.2");
        check!("198.19.255.255");
        // make sure addresses reserved for protocol assignment are not global
        check!("192.0.0.0");
        check!("192.0.0.255");
        check!("192.0.0.100");
        // make sure reserved addresses are not global
        check!("240.0.0.0");
        check!("251.54.1.76");
        check!("254.255.255.255");
        // make sure shared addresses are not global
        check!("100.64.0.0");
        check!("100.127.255.255");
        check!("100.100.100.0");

        check!("::", unspec);
        check!("::1", loopback);
        check!("::0.0.0.2", global);
        check!("1::", global);
        check!("fc00::");
        check!("fdff:ffff::");
        check!("fe80:ffff::");
        check!("febf:ffff::");
        check!("fec0::", global);
        check!("ff01::", multicast);
        check!("ff02::", multicast);
        check!("ff03::", multicast);
        check!("ff04::", multicast);
        check!("ff05::", multicast);
        check!("ff08::", multicast);
        check!("ff0e::", global | multicast);
        check!("2001:db8:85a3::8a2e:370:7334", doc);
        check!("102:304:506:708:90a:b0c:d0e:f10", global);
    }
}
