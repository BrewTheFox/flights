use std::{env, fs, path};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub key: String,
    pub id: String,
    pub ip: String
}

#[derive(Serialize, Deserialize)]
struct Devices {
    pub device_list: Vec<Device>
}

fn validate_device(dev: &Device) -> bool {
    if dev.key.len() != 16 {
        eprintln!("Key length must be 16.");
        return false;
    }
    
    if dev.id.len() != 22 {
        eprintln!("The length of the device id must be 22.");
        return false;
    }

    return true;
}

fn write_config(data:&Devices) -> bool {
    let mut config_dir = env::home_dir().unwrap();
    config_dir.push(".config/flights/bulbs.json");

    let configuration = serde_json::to_string(&data);

    if configuration.is_err() {
        eprintln!("Couldn't serialize configuration file.");
        return false;
    }

    let configuration = configuration.unwrap();
    let status = fs::write(config_dir, configuration);
    if status.is_err() {
        eprintln!("Couldn't write configuration file.");
        return false;
    }
    return true;
}

fn create_config(dir:&path::PathBuf) -> bool {
    let config_dir = dir.parent().unwrap();
    let status = fs::create_dir_all(config_dir);

    if status.is_err() {
        eprintln!("Couldn't create configuration directory at: {}", config_dir.to_str().unwrap());
        return false;
    }

    let status = fs::File::create(&dir);

    if status.is_err() {
        eprintln!("Couldn't create configuration file at: {}", dir.to_str().unwrap());
        return false;
    }


    let config = Devices {device_list: Vec::new()};
    if write_config(&config) == false {
        return false;
    }

    return true;
}

fn check_integrity() -> bool{
    let home_dir = env::home_dir();

    if home_dir.is_none() {
        eprintln!("Can't do anything! Couldn't find home dir!");
        return false;
    }

    let home_dir = home_dir.unwrap(); // I'd be surprised if this somehow managed to fail...
    if !home_dir.exists() {
        eprintln!("Can't do anything! Home dir does not exist!");
        return false;
    }

    let mut config_dir = home_dir;
    config_dir.push(".config/flights/bulbs.json");

    if !config_dir.exists() {
        if !create_config(&config_dir) {
            return false;
        }
    }

    return true;
}

fn load_devices() -> Option<Devices> {
    let mut config_dir = env::home_dir().unwrap(); // Can unwrap because its existance was checked in check_integrity
    config_dir.push(".config/flights/bulbs.json");

    if !config_dir.exists() {
        eprintln!("Can't do anything! Config dir was deleted!");
        return None;
    }

    let contents = fs::read_to_string(config_dir);
    if contents.is_err() {
        eprintln!("Couldn't read config contents!");
        return None;
    }

    let contents = contents.unwrap();
    let data: Result<Devices, serde_json::Error> = serde_json::from_str(&contents);

    if data.is_err() {
        eprintln!("Error deserializing contents of config file!");
        return None;
    }

    Some(data.unwrap())
}

pub fn add_device(name: String, key:String, id:String, ip:String) -> bool{
    if !check_integrity() {
        return false;
    }

    let dev = Device {
        name:name,
        key:key,
        id:id,
        ip:ip
    };

    if validate_device(&dev) == false {
        return false;
    }

    let devices: Option<Devices> = load_devices();

    if devices.is_none() {
        return false;
    }

    let mut device_list = devices.unwrap().device_list;

    for cdev in &device_list {
        if cdev.name == dev.name {
            eprintln!("There's already a device with that name! remove it first!");
            return false;
        }
    }

    device_list.push(dev);

    let data = Devices {
        device_list:device_list
    };

    if !write_config(&data) {
        return false;
    }
    println!("Device added successfully!");
    return true;
}

pub fn remove_device(name: String) -> bool {
    if !check_integrity() {
        return false;
    }

    let devices: Option<Devices> = load_devices();

    if devices.is_none() {
        return false;
    }

    let device_list = devices.unwrap().device_list;
    let mut new_device_list : Vec<Device> = Vec::new();
    let mut found = false;

    for dev in device_list {
        if dev.name == name {
            found = true;
            continue;
        }
        new_device_list.push(dev);
    }
    if !found {
        eprintln!("Couldn't find device with name: {}. no changes were made.", name);
        return false;
    }

    let data = Devices {
        device_list:new_device_list
    };

    if !write_config(&data) {
        return false;
    }

    println!("Device {} removed successfully!", name);

    return true;
}

pub fn list_devices() -> bool {

    if !check_integrity() {
        return false;
    }

    let devices: Option<Devices> = load_devices();

    if devices.is_none() {
        return false;
    }

    let device_list = devices.unwrap().device_list;
    println!("Name, ID");
    println!("------------------------------------------------------");
    for device in device_list {
        println!("{}, {}", device.name, device.id);
        println!("------------------------------------------------------");
    }
    return true;
}

pub fn fetch_device(name: String) -> Option<Device> {
    if !check_integrity() {
        return None;
    }

    let devices: Option<Devices> = load_devices();

    if devices.is_none() {
        return None;
    }

    let device_list = devices.unwrap().device_list;

    for dev in device_list {
        if dev.name == name {
            return Some(dev);
        }
    }

    eprintln!("Couldn't find device with the name: {}", name);
    return None;
}