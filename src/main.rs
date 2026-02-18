use clap::{Parser, Subcommand};
use std::{net::IpAddr, process::exit, str::FromStr};

mod tuya;
mod config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about="Add light bulb to your local storage.")]
    Add {
        #[arg(short, long, help="device name")]
        name: String,
        #[arg(short, long, help="local key")]
        key: String,
        #[arg(long, help="device id")]
        id: String,
        #[arg(long, help="ip address")]
        ip: String,
    },

    #[command(about="Remove a light bulb from your local storage.")]
    Remove {
        #[arg(short, long, help="device name")]
        name: String
    },

    #[command(about="List the name of all devices inside your local storage.")]
    List,

    #[command(about="Turn on a light bulb.")]
    On {
        #[arg(short, long, help="device name")]
        name:String
    },

    #[command(about="Turn off a light bulb.")]
    Off {
        #[arg(short, long, help="device name")]
        name:String
    },

    #[command(about="Change the color of a light bulb.")]
    Color {
        #[arg(short, long, help="device name")]
        name:String,
        #[arg(short=char::from_str("c").unwrap(), long, help="Hex Color")]
        hex_color:String
    },

    #[command(about="Set the color of a light bulb to white.")]
    White {
        #[arg(short, long, help="device name")]
        name: String
    },

    #[command(about="Set the brightness of a light bulb.")]
    Brightness {
        #[arg(short, long, help="device name")]
        name: String,
        #[arg(short, long, help="number between 1 - 100.")]
        percent: u8
    }, 

    #[command(about="Get the status of a device.")]
    Status {
        #[arg(short, long, help="device name")]
        name: String,
    }
}


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { name, key, id, ip} => {
            config::add_device(name, key, id, ip);
        }

        Commands::Remove { name } => {
            config::remove_device(name);
        }

        Commands::List => {
            config::list_devices();
        }

        Commands::On { name } => {
            let device = config::fetch_device(name);

            if device.is_none() {
                exit(2)
            }

            let device = device.unwrap();

            match IpAddr::from_str(&device.ip) {
                Ok(address) => tuya::light_switch(device.id, device.key, address, true),
                Err(e) => eprintln!("Fatal error decoding IP Address {}", e),
            }
        }

        Commands::Off { name } => {
            let device = config::fetch_device(name);

            if device.is_none() {
                exit(2)
            }

            let device = device.unwrap();
            
            match IpAddr::from_str(&device.ip) {
                Ok(address) => tuya::light_switch(device.id, device.key, address, false),
                Err(e) => eprintln!("Fatal error decoding IP Address {}", e),
            }
        }

        Commands::Color { name, hex_color } => {
            let device = config::fetch_device(name);

            if device.is_none() {
                exit(2)
            }

            let device = device.unwrap();

            match IpAddr::from_str(&device.ip) {
                Ok(address) => tuya::color_change(device.id, device.key, address, hex_color),
                Err(e) => eprintln!("Fatal error decoding IP Address {}", e),
            }
        }

        Commands::White { name } => {
            let device = config::fetch_device(name);

            if device.is_none() {
                exit(2)
            }

            let device = device.unwrap();

            match IpAddr::from_str(&device.ip) {
                Ok(address) => tuya::set_white(device.id, device.key, address),
                Err(e) => eprintln!("Fatal error decoding IP Address {}", e),
            }
        }

        Commands::Brightness { name, percent } => {
            if percent > 100 || percent < 1 {
                eprintln!("Percent must be a number between 1 and 100.");
                exit(2);
            }

            let device = config::fetch_device(name);

            if device.is_none() {
                exit(2)
            }

            let device = device.unwrap();
            match IpAddr::from_str(&device.ip) {
                Ok(address) => tuya::set_brightness(device.id, device.key, address, percent),
                Err(e) => eprintln!("Fatal error decoding IP Address {}", e),
            }
        },

        Commands::Status { name } => {
            let device = config::fetch_device(name);
            if device.is_none() {
                exit(2)
            }

            let device = device.unwrap();
            match IpAddr::from_str(&device.ip) {
                Ok(address) => tuya::get_info_packets(device.id, device.key, address, device.name),
                Err(e) => eprintln!("Fatal error decoding IP Address {}", e),
            }
        }
    }
}
