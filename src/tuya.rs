use std::{collections::HashMap, net::IpAddr, time::SystemTime};
use rust_tuyapi::{self, Payload, PayloadStruct, TuyaDevice};
use serde_json::{Value, json};
use color_space::{Rgb, Hsv};

pub fn light_switch(id:String, localkey:String, ip:IpAddr, state:bool) {
    let mut dps = HashMap::new();
    dps.insert("20".to_string(), json!(state));
    send_change_packet(dps, id, localkey, ip);
}

pub fn color_change(id:String, localkey:String, ip:IpAddr, color:String) {
    let colour = hex_to_tuya(&color);
    if colour.is_some() {
        let mut dps = HashMap::new();
        dps.insert("21".to_string(), json!("colour"));
        dps.insert("24".to_string(), json!(colour));
        send_change_packet(dps, id, localkey, ip);
    }
    else {
        eprintln!("Invalid HEX color.")
    }
}

pub fn set_white(id:String, localkey:String, ip:IpAddr) {
    let mut dps = HashMap::new();
    dps.insert("21".to_string(), json!("white"));
    send_change_packet(dps, id, localkey, ip);
}

pub fn set_brightness(id:String, localkey:String, ip:IpAddr, brightness:u8) {
    let mut dps = HashMap::new();
    dps.insert("22".to_string(), json!(brightness as u16 * 10));
    send_change_packet(dps, id, localkey, ip);
}

pub fn get_info_packets(id:String, localkey:String, ip:IpAddr, name:String) {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32;
    let dps = HashMap::new();
    let payload = Payload::Struct(PayloadStruct{
       dev_id: id.clone(),
       gw_id: Some(id),
       uid: None,
       t: Some(current_time),
       dp_id: None,
       dps: Some(dps),
       });
    
    match TuyaDevice::create("ver3.3", Some(&localkey), ip) {
        Ok(tuya_device) => {
            match tuya_device.get(payload, 0) {
                Ok(data) => {
                    for item in data {
                        let val: Result<Value, serde_json::Error> = serde_json::from_str(&item.payload.to_string());
                        match val {
                            Ok(val) => {
                                println!("{name}");
                                println!("----------------------");
                                println!("Satus: {}", bool_to_turn_status(val["dps"]["20"].clone()));
                                println!("Mode: {}", val["dps"]["21"].clone().as_str().unwrap().replace("\"", ""));
                                println!("Brightness: {}", (val["dps"]["22"].clone().as_u64().unwrap() / 10_u64).to_string());
                            }
                            Err(_) => eprintln!("Data cannot be serialized.")
                        }
                    }
                }
                Err(e) => eprintln!("Failed to retrieve information: {:?}", e),
            }
        }
        Err(e) => eprintln!("Failed to create Tuya device: {:?}", e),
    }
}

fn send_change_packet(dps:HashMap<String, Value>, id:String, localkey:String, ip:IpAddr) {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u32;
    let payload = Payload::Struct(PayloadStruct{
       dev_id: id.clone(),
       gw_id: Some(id),
       uid: None,
       t: Some(current_time),
       dp_id: None,
       dps: Some(dps),
       });
    
    match TuyaDevice::create("ver3.3", Some(&localkey), ip) {
        Ok(tuya_device) => {
            match tuya_device.set(payload, 0) {
                Ok(_) => println!("Device command sent successfully"),
                Err(e) => eprintln!("Failed to send command: {:?}", e),
            }
        }
        Err(e) => eprintln!("Failed to create Tuya device: {:?}", e),
    }
}

fn bool_to_turn_status(turn:Value) -> &'static str {
    if turn == true {
        return "Turned On";
    }
    return "Turned Off";
}

fn hex_to_tuya(hex: &str) -> Option<String> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    let rgb = Rgb::new(r as f64, g as f64, b as f64);
    let hsv = Hsv::from(rgb);
    Some(format!("{:04x}{:04x}{:04x}", hsv.h as u16, (hsv.s as f32 * 1000_f32) as u16, (hsv.v as f32 * 1000_f32) as u16))
}
