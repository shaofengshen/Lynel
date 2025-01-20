mod vehicle;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use rppal::i2c::I2c;
use vehicle::Vehicle;
use ctrlc;


// Define the color command
const COLOR_RED: u8 = 0x01;
const COLOR_GREEN: u8 = 0x02;
const COLOR_BLUE: u8 = 0x03;
const COLOR_YELLOW: u8 = 0x04;
const COLOR_PURPLE: u8 = 0x05;
const COLOR_CYAN: u8 = 0x06;
const COLOR_WHITE: u8 = 0x07;

fn main() {
    let i2c = I2c::new().unwrap();  // Create an I2C instance
    let vehicle = Arc::new(Mutex::new(Vehicle::new(i2c)));    // Set up the vehicle

    // Set up Ctrl-C handler
    let vehicle_clone = Arc::clone(&vehicle);    // Clone the vehicle
    ctrlc::set_handler(move|| {
        println!("Ctrl-C pressed!");
        vehicle_clone.lock().unwrap().led_light_off();
        std::process::exit(0);    // Exit the program
    }).expect("Error setting Ctrl-C handler");

    let colors = [COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_YELLOW, COLOR_PURPLE, COLOR_CYAN, COLOR_WHITE]; // 颜色数组

    loop {
        for &color in &colors {
            vehicle.lock().unwrap().led_light_on(color);
            thread::sleep(Duration::from_secs(1));
        }

        vehicle.lock().unwrap().led_light_off();
        thread::sleep(Duration::from_secs(1));       
    }
}