use std::thread;
use std::time::Duration;
use rppal::i2c::I2c;

const DEVICE_ADDRESS: u16 = 0x2B; // 设备地址
const LED_ADDRESS: u8 = 0x03; // LED 灯的地址

// 定义颜色命令
const COLOR_RED: u8 = 0x01;
const COLOR_GREEN: u8 = 0x02;
const COLOR_BLUE: u8 = 0x03;
const COLOR_YELLOW: u8 = 0x04;
const COLOR_PURPLE: u8 = 0x05;
const COLOR_CYAN: u8 = 0x06;
const COLOR_WHITE: u8 = 0x07;

fn main() {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(DEVICE_ADDRESS).unwrap(); // 设置设备地址

    let colors = [COLOR_RED, COLOR_GREEN, COLOR_BLUE, COLOR_YELLOW, COLOR_PURPLE, COLOR_CYAN, COLOR_WHITE]; // 颜色数组

    loop {
        for &color in &colors {
            // 设置颜色，开关状态为 1（打开）
            let command = [LED_ADDRESS, 0x01, color]; // 地址、开关、颜色
            i2c.write(&command).unwrap();
            println!("灯颜色已更换");
            thread::sleep(Duration::from_secs(1));
        }

        // 熄灭灯，开关状态为 0（关闭）
        let command_off = [LED_ADDRESS, 0x00, 0x00]; 
        i2c.write(&command_off).unwrap();
        println!("灯已熄灭");
        thread::sleep(Duration::from_secs(1)); // 添加 50ms 延时
    }
}

/*
use std::thread;
use std::time::Duration;
use rppal::i2c::I2c;

const DEVICE_ADDRESS: u16 = 0x2B; // 设备地址
const MOTOR_ADDRESS: u8 = 0x01; // 电机地址
const LED_ADDRESS: u8 = 0x03; // LED 灯的地址
const LED_ON: u8 = 0x01; // 点亮灯的命令
const LED_OFF: u8 = 0x00; // 熄灭灯的命令

// 定义电机号
const MOTOR_1_ID: u8 = 0x00;
const MOTOR_2_ID: u8 = 0x01;
const MOTOR_3_ID: u8 = 0x02;
const MOTOR_4_ID: u8 = 0x03;

// 定义电机方向
const MOTOR_FORWARD: u8 = 0x00; // 电机向前的方向
const MOTOR_BACKWARD: u8 = 0x01; // 电机向后的方向

// 定义电机速度
const MOTOR_SPEED: u8 = 50; // 电机速度

// 定义颜色命令
const COLOR_GREEN: u8 = 0x02; // 绿色
const COLOR_YELLOW: u8 = 0x04; // 黄色

fn main() {
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(DEVICE_ADDRESS).unwrap(); // 设置设备地址

    loop {
        // 控制电机向前运行 2 秒，灯为绿色
        for motor_id in &[MOTOR_1_ID, MOTOR_2_ID, MOTOR_3_ID, MOTOR_4_ID] {
            let motor_command_forward = [MOTOR_ADDRESS, *motor_id, MOTOR_FORWARD, MOTOR_SPEED]; // 电机号、方向、速度
            i2c.write(&motor_command_forward).unwrap();
            println!("电机 {} 向前运行，速度 {}", motor_id, MOTOR_SPEED);
        }
        
        // 设置灯为绿色
        let command_green = [LED_ADDRESS, LED_ON, COLOR_GREEN]; // 地址、开关、颜色
        i2c.write(&command_green).unwrap();
        thread::sleep(Duration::from_secs(2));
        
        // 设置灯为黄色
        let command_yellow = [LED_ADDRESS, LED_ON, COLOR_YELLOW]; // 地址、开关、颜色
        i2c.write(&command_yellow).unwrap();
        thread::sleep(Duration::from_secs(2));

        // 熄灭灯
        let command_off = [LED_ADDRESS, LED_OFF, 0x00]; // 地址、开关、颜色（颜色可以是任意值）
        i2c.write(&command_off).unwrap();
        println!("灯已熄灭");
        thread::sleep(Duration::from_secs(1));
    }
}
*/