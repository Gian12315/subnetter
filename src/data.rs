use crate::class::Class;
use crate::opt::Opt;
use crate::row::Row;
use std::fmt;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Data {
    ip: Ipv4Addr,
    mask: u8,
    subnetworks: u8,
    submask: u8,
    hosts: u32,
    jump: u8,
    class: Class,
    generated: u8,
}

impl Data {
    pub fn new(config: &Opt) -> Data {
        let mut n = 0;
        while 2i32.pow(n) < config.subnetworks as i32 {
            n += 1;
        }

        let submask = config.mask + (n as u8);
        let m = 32 - submask;

        let hosts = 2u32.pow(m as u32) - 2;

        let mut jump_mask = submask;
        while jump_mask > 8 {
            jump_mask -= 8;
        }

        let mut bits: [&str; 8] = ["0"; 8];
        for bit in 0..jump_mask {
            bits[bit as usize] = "1";
        }

        let string = bits.join("");
        let mod_oct_bin = u8::from_str_radix(&string, 2).expect("Failed to convert to u8");
        let jump = (255 - mod_oct_bin) + 1; // +1 because 255 is the max value for an u8, but we have to subtract from 256

        let class = Class::new(config.mask);
        Data {
            ip: config.ip,
            mask: config.mask,
            subnetworks: config.subnetworks,
            submask,
            hosts,
            jump,
            class,
            generated: 0,
        }
    }
}

impl Iterator for Data {
    type Item = Row;
    fn next(&mut self) -> Option<Self::Item> {
        if self.generated == self.subnetworks {
            None
        } else {
            let jumped = self.generated * self.jump;
            let next_jump = (self.generated + 1) * self.jump;

            let mut octects = self.ip.octets();
            match self.class {
                Class::A => {
                    octects[1] += jumped;
                }
                Class::B => {
                    octects[2] += jumped;
                }
                Class::C => {
                    octects[3] += jumped;
                }
            }

            let network = Ipv4Addr::new(octects[0], octects[1], octects[2], octects[3]);
            let first_ip = Ipv4Addr::new(octects[0], octects[1], octects[2], octects[3] + 1);

            match self.class {
                Class::A => {
                    octects[1] = next_jump - 1;
                    octects[2] = 255;
                    octects[3] = 254;
                }
                Class::B => {
                    octects[2] = next_jump - 1;
                    octects[3] = 254
                }
                Class::C => {
                    octects[3] = next_jump - 2;
                }
            }

            let last_ip = Ipv4Addr::new(octects[0], octects[1], octects[2], octects[3]);
            let broadcast = Ipv4Addr::new(octects[0], octects[1], octects[2], octects[3] + 1);

            self.generated += 1;

            Some(Row::new(network, first_ip, last_ip, broadcast))
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IP: {}
MASK: {}
SUBNETWORKS: {}
SUBMASK: {}
HOSTS: {}
JUMP: {}
CLASS: {}",
            self.ip, self.mask, self.subnetworks, self.submask, self.hosts, self.jump, self.class
        )
    }
}
