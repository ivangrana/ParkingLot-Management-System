// parking_lot.rs
use crate::car::Car;

pub struct ParkingLot {
    cars: Vec<Car>,
}

impl ParkingLot {
    // Create a new ParkingLot instance with an empty vector of cars.
    pub fn new() -> ParkingLot {
        ParkingLot { cars: Vec::new() }
    }

    // Insert a car into the parking lot and print a success message.
    pub fn insert_car(&mut self, new_car: Car) {
        // Check if the car is already in the parking lot
        //iter().any() method iterates through the cars vector..
        //in the parking lot has the same make, model, and car plate as the new car.
        if self.cars.iter().any(|car| {
            car.get_make() == new_car.get_make()
            && car.get_model() == new_car.get_model()
            && car.get_car_plate() == new_car.get_car_plate()
        }) {
            println!("Duplicate car found. Car not parked.");
        } else {
            // Insert the car into the parking lot
            self.cars.push(new_car);
            println!("Car parked successfully!");
        }
    }

    // Remove a car from the parking lot based on make, model, and number plate.
    // Return an Option containing the removed car or None if not found.
    pub fn remove_car(&mut self, make: &str, model: &str, car_plate: &str) -> Option<Car> {
        if let Some(index) = self.cars.iter().position(|car| {
            car.get_make() == make && car.get_model() == model && car.get_car_plate() == car_plate
        }) {
            let removed_car = self.cars.remove(index);
            println!("Car removed successfully!");
            Some(removed_car)
        } else {
            println!("Car not found in the parking lot.");
            None
        }
    }

    // Display the list of cars in the parking lot.
    pub fn display_parking_lot(&self) {
        println!("Cars in the parking lot:");
        for car in &self.cars {
            println!(
                "{} | {} | {}",
                car.get_make(),
                car.get_model(),
                car.get_car_plate()
            );
        }
    }
}
