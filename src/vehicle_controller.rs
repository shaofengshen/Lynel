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

    pub fn get_ultrasonic_distance(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        // Enable ultrasonic sensor
        self.vehicle.enable_ultrasonic_sensor();
        // Read ultrasonic sensor
        let distance = match self.vehicle.read_ultrasonic() {
            Ok(d) => {
                // Disable ultrasonic sensor
                self.vehicle.disable_ultrasonic_sensor()?;
                Ok(d)
            }
            Err(e) => {
                // Disable ultrasonic sensor, even if reading failed
                self.vehicle.disable_ultrasonic_sensor()?;
                Err(e)
            }
        }?;

        Ok(distance)
    }
}
