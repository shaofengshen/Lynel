use rppal::i2c::I2c;

const DEVICE_ADDRESS: u16 = 0x2B; // 设备地址
const MOTOR_ADDRESS: u8 = 0x01; // 电机地址
const LED_ADDRESS: u8 = 0x03; // LED 灯的地址
const MOTOR_FORWARD: u8 = 0x01; // 电机正转
const MOTOR_BACKWARD: u8 = 0x02; // 电机反转
// TODO: 电机停止, 确认电机停止的命令
const MOTOR_SPEED: u8 = 50; // 电机默认速度

pub struct Vehicle {
    i2c: I2c,
}

impl Vehicle {
    pub fn new(i2c: I2c) -> Self {
        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(DEVICE_ADDRESS).unwrap(); // 设置设备地址
        Vehicle { i2c }
    }

    pub fn start_motor(&mut self, motor_id: u8) {
        let motor_command_forward = [MOTOR_ADDRESS, motor_id, MOTOR_FORWARD, MOTOR_SPEED];
        self.i2c.write(&motor_command_forward).unwrap();
        println!("Motor {} started forward, speed: {}", motor_id, MOTOR_SPEED);
    }

    pub fn led_on(&mut self, color: u8) {
        let led_command = [LED_ADDRESS, 0x01, color];
        self.i2c.write(&led_command).unwrap();
        println!("LED on, color: {}", color);
    }

    pub fn led_off(&mut self) {
        let led_command = [LED_ADDRESS, 0x00, 0x00];
        self.i2c.write(&led_command).unwrap();
        println!("LED off");
    }
}