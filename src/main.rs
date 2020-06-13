use csv::Writer;
use std::io;

mod class;
mod data;
mod opt;
mod row;

use data::Data;
use opt::Opt;
use structopt::StructOpt;

fn main() -> io::Result<()> {
    let config = Opt::from_args();
    let data = Data::new(&config);
    if let Some(path) = &config.path {
        let mut writer = Writer::from_path(&path)?;
        for row in data {
            writer.serialize(&row)?;
        }
        writer.flush()?;
    } else {
        println!("Network - First IP - Last IP - Broadcast");
        for row in data {
            println!("{}", row);
        }
    }
    Ok(())
}
