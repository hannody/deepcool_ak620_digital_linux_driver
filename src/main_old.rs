extern crate hidapi;
use hidapi::HidApi;


// fn main() {
//     let api = HidApi::new().expect("Failed to initialize HIDAPI");

//     for device_info in api.device_list() {
//         println!("\n------------------------"); // Dashed line before each device
//         println!("{:#?}", device_info);      // Pretty print device info

//         if let Some(manufacturer) = device_info.manufacturer_string() {
//             println!("Manufacturer: {}", manufacturer);
//         } else {
//             println!("Manufacturer: Unknown");
//         }
//         println!("------------------------\n"); // Dashed line after each device
//     }
// }

use std::thread::sleep;
use std::time::Duration;
use sysinfo::Components;

const VENDOR_ID: u16 = 13875; // DeepCool's Vendor ID (replace with your actual value)
const PRODUCT_ID: u16 = 2; // DeepCool AK620 Product ID (replace with your actual value)

fn get_data(value: u8, mode: &str) -> Vec<u8> {
    let mut base_data = vec![16; 64];
    let numbers = value.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

    base_data[2] = (value - 1) / 10 + 1;

    match mode {
        "util" => base_data[1] = 0,
        "start" => base_data[1] = 170,
        "temp" => base_data[1] = 19,
        _ => {} // Invalid mode, do nothing
    }

    for (i, &digit) in numbers.iter().enumerate() {
        base_data[3 + i] = digit;
    }

    base_data
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let device = api.open(VENDOR_ID, PRODUCT_ID)?;
    let components = Components::new_with_refreshed_list();
    for component in &components {
        if component.label().contains("k10temp Tccd1"){
            println!("{} {}°C", component.label(), component.temperature().round());
            // Display '42' on the AK620
            device.write(&get_data(component.temperature().round() as u8, "temp"))?;
        }
    }

    // Add a short delay to ensure the display updates
    sleep(Duration::from_millis(500)); 

    Ok(())
}
