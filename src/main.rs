mod car;
mod parking_lot;

use std::io;

use car::Car;
use parking_lot::ParkingLot;

fn main() {
    // Create a parking lot instance.
    let mut parking_lot: ParkingLot = parking_lot::ParkingLot::new();

    loop {
        println!("[1] - Add a car to the parking lot");
        println!("[2] - Display the cars of the parking lot");
        println!("[3] -  Remove a car from the parking lot");
        println!("[4] -  Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                // Create an example car.
                let make = get_user_input("Enter car make: ");
                let model = get_user_input("Enter car model: ");
                let year = get_user_input("Enter car year: ");
                let car_example = Car::new(&make, &model, year.parse().unwrap());
                parking_lot.insert_car(car_example);
            }
            2 => {
                // Display the contents of the parking lot.
                parking_lot.display_parking_lot();
            }
            3 => {
                // Remove a car from the parking lot based on make, model, and year.
                let make = get_user_input("Enter car make to remove: ");
                let model = get_user_input("Enter car model to remove: ");
                let year = get_user_input("Enter car year to remove: ");
                parking_lot.remove_car(&make, &model, year.parse().unwrap());
            }
            4 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
