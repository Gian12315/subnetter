use serde::Serialize;
use std::fmt;
use std::net::Ipv4Addr;

#[derive(Debug, Serialize)]
pub struct Row {
    #[serde(rename = "Network")]
    net: Ipv4Addr,
    #[serde(rename = "First IP")]
    first_ip: Ipv4Addr,
    #[serde(rename = "Last IP")]
    last_ip: Ipv4Addr,
    #[serde(rename = "Broadcast")]
    broadcast: Ipv4Addr,
}

impl Row {
    pub fn new(net: Ipv4Addr, first_ip: Ipv4Addr, last_ip: Ipv4Addr, broadcast: Ipv4Addr) -> Row {
        Row {
            net,
            first_ip,
            last_ip,
            broadcast,
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} - {} - {}",
            self.net, self.first_ip, self.last_ip, self.broadcast
        )
    }
}
