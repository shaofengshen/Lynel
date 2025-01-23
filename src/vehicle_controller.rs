use crate::vehicle::Vehicle;
use rppal::i2c::I2c;

pub struct VehicleController {
    pub vehicle: Vehicle,
}

impl VehicleController {
    pub fn new(i2c: I2c) -> Self {
        VehicleController {
            vehicle: Vehicle::new(i2c),
        }
    } 

    pub fn forward(&mut self) {
        for motor_id in 0..4 {
            self.vehicle.start_motor(motor_id);
        }
    }

    pub fn backward(&mut self) {
        for motor_id in 0..4 {
            self.vehicle.start_motor(motor_id);
        }
    }

    pub fn stop(&mut self) {
        for motor_id in 0..4 {
            self.vehicle.stop_motor(motor_id);
        }
    }
}
