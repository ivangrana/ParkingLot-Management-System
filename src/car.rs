// car.rs

pub struct Car {
    make: String,
    model: String,
    car_plate: String // number plate of the car
}

impl Car {
    // Create a new car instance with the given make, model, and  plate.
    pub fn new(make: &str, model: &str, car_plate: &str) -> Car {
        Car {
            make: make.to_string(),
            model: model.to_string(),
            car_plate:car_plate.to_string(),
        }
    }

    // Get the make of the car.
    pub fn get_make(&self) -> &str {
        &self.make
    }

    // Get the model of the car.
    pub fn get_model(&self) -> &str {
        &self.model
    }

    // Get the car_plate of the car.
    pub fn get_car_plate(&self) -> &str {
        &self.car_plate
    }
}
