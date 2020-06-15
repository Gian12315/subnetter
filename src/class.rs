use std::fmt;
use std::process;

#[derive(Debug)]
pub enum Class {
    A,
    B,
    C,
}

impl Class {
    pub fn new(mask: u8) -> Class {
        match mask {
            8 => Class::A,
            16 => Class::B,
            24 => Class::C,
            _ => {
                eprintln!("Please use a valid network mask");
                process::exit(1);
            }
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            match self {
                Class::A => "A",
                Class::B => "B",
                Class::C => "C",
            }
        })
    }
}
