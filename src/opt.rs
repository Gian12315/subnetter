use std::net::Ipv4Addr;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Subnetter",
    about = "Subnet your network without all the hassle"
)]
pub struct Opt {
    /// IP to be subnetted
    pub ip: Ipv4Addr,
    /// Mask of the IP to be subnetted
    pub mask: u8,
    /// Number of subnetworks to be computed
    pub subnetworks: u8,

    /// Save to a csv file
    #[structopt(short, parse(from_os_str))]
    pub path: Option<PathBuf>,

    /// Whenever print debug data or not
    #[structopt(short)]
    pub debug: bool,
}
