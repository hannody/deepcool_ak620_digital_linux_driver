use hidapi::HidApi;
use std::{thread::sleep, time::Duration};
use sysinfo::Components;

const VENDOR_ID: u16 = 13875; // DeepCool's Vendor ID
const PRODUCT_ID: u16 = 2;   // DeepCool AK620 Product ID

fn get_data(value: u8) -> Vec<u8> {
    let mut base_data = vec![0; 64];
    base_data[0] = 16;
    base_data[1] = 19;

    base_data[2] = value / 10;
    base_data[3] = 0; // Tens digit
    base_data[4] = value / 10; // Tens digit
    base_data[5] = value % 10; // Ones digit

    return base_data;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let device = api.open(VENDOR_ID, PRODUCT_ID)?;

    loop {
        // Refresh the component list each iteration
        let components = Components::new_with_refreshed_list();

        // Find the CPU temperature sensor
        if let Some(component) = components.iter().find(|c| c.label().contains("k10temp Tccd1")) {
            let temp = component.temperature().round() as u8;

            // Check for valid temperature range before displaying
            if temp > 0 && temp < 100 {
                device.write(&get_data(temp))?;
            } else {
                println!("Invalid CPU temperature: {}", temp); // Handle invalid readings
            }
        } else {
            println!("CPU temperature sensor not found.");
        }

        sleep(Duration::from_secs(3)); // Update every second (adjust as needed)
    }
}

