struct Car { // Define a struct representing a car with make, model, and year.
    make: String,
    model: String,
    year: i32, // 
}

struct ParkingLot { // // Define a struct representing a parking lot containing a vector of cars.
    cars: Vec<Car>,
}

impl ParkingLot { // Create a new ParkingLot instance with an empty vector of cars.
    fn new() -> ParkingLot {
        ParkingLot { cars: Vec::new() }
    }

    fn insert_car(&mut self, car: Car) { // Insert a car into the parking lot and print a success message.
        self.cars.push(car);
        println!("Car parked successfully!");
    }

    fn remove_car(&mut self, make: &str, model: &str, year: i32) -> Option<Car> {
        if let Some(index) = self
            .cars
            .iter()
            .position(|car| car.make == make && car.model == model && car.year == year)
        {
            let removed_car = self.cars.remove(index);
            println!("Car removed successfully!");
            Some(removed_car)
        } else {
            println!("Car not found in the parking lot.");
            None
        }
    }

    fn display_parking_lot(&self) {
        println!("Cars in the parking lot:");
        for car in &self.cars {
            println!("{} {} {}", car.make, car.model, car.year);
        }
    }
}

fn main() {
    let car_example: Car = Car {
        make: "Toyota".to_string(),
        model: "Corolla".to_string(),
        year: 2013,
    };

    let mut parking_lot: ParkingLot = ParkingLot::new();
    parking_lot.insert_car(car_example);
    parking_lot.display_parking_lot();
    parking_lot.remove_car("Toyota", "Corolla", 2013);
}
