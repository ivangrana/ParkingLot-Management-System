// main.rs
mod car;
mod parking_lot;

use car::Car;
use parking_lot::ParkingLot;

fn main() {
    // Create an example car.
    let car_example = Car::new("Ford","Mustang",2019);

    // Create a parking lot instance.
    let mut parking_lot: ParkingLot = parking_lot::ParkingLot::new();

    // Insert the example car into the parking lot.
    parking_lot.insert_car(car_example);

    // Display the contents of the parking lot.
    parking_lot.display_parking_lot();

    // Remove a car from the parking lot based on make, model, and year.
    parking_lot.remove_car("Toyota", "Corolla", 2013);

}
