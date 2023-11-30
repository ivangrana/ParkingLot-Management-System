// car.rs

pub struct Car {
    make: String,
    model: String,
    year: i32, // Year of the car
}

impl Car {
    // Create a new car instance with the given make, model, and year.
    pub fn new(make: &str, model: &str, year: i32) -> Car {
        Car {
            make: make.to_string(),
            model: model.to_string(),
            year,
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

    // Get the year of the car.
    pub fn get_year(&self) -> i32 {
        self.year
    }
}
