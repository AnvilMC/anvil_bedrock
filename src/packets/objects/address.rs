use std::net::SocketAddr;

#[derive(Debug, packet_derive::Biscuit, Clone, PartialEq, Copy)]
pub struct Address {
    pub ip_type: u8,
    pub ip_bytes: [u8; 4],
    pub port: u16
}

impl From<&SocketAddr> for Address {
    fn from(e: &SocketAddr) -> Self {
        match e {
            SocketAddr::V4(a) => {
                Self {
                    ip_type: 4,
                    ip_bytes: a.ip().octets(),
                    port: a.port(),
                }
            }
            SocketAddr::V6(_) => unreachable!()
        }
    }
}