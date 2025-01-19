mod vehicle;
use std::thread;
use std::time::Duration;
use rppal::i2c::I2c;
use vehicle::Vehicle;


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
    let mut vehicle = Vehicle::new(i2c);    // Set up the vehicle

    let colors = [COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_YELLOW, COLOR_PURPLE, COLOR_CYAN, COLOR_WHITE]; // 颜色数组

    loop {
        for &color in &colors {
            vehicle.led_on(color);
            thread::sleep(Duration::from_secs(1));
        }

        vehicle.led_off();
        thread::sleep(Duration::from_secs(1));       
    }
}