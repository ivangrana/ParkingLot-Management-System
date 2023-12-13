# Rust Parking Lot Management System

This Rust project demonstrates a simple parking lot management system with two main components: `Car` and `ParkingLot`. The goal is to illustrate basic Rust concepts such as structs, modules, ownership, and functional programming.

## Project Structure

The project is organized into three main files within the `src` directory:

1. **`car.rs`**: Contains the definition of the `Car` struct and its associated methods.
2. **`parking_lot.rs`**: Defines the `ParkingLot` struct and its functionality for managing cars.
3. **`main.rs`**: The entry point of the application where an example car is created and interactions with the parking lot are demonstrated.

## Components

### Car Struct (`car.rs`)

The `Car` struct represents a car with the following fields:

- `make`: The manufacturer or brand of the car (String).
- `model`: The specific model of the car (String).
- `car_plate`: The plate of the car (String).

**Methods:**

- `new(make: &str, model: &str, car_plate: &str) -> Car`: Creates a new `Car` instance.
- `get_make() -> &str`: Returns the make of the car.
- `get_model() -> &str`: Returns the model of the car.
- `get_car_plate() -> &str`: Returns the car plate of the car.

### ParkingLot Struct (`parking_lot.rs`)

The `ParkingLot` struct manages a collection of cars using a vector. It has the following features:

- `cars`: A vector of `Car` instances.

**Methods:**

- `new() -> ParkingLot`: Creates a new `ParkingLot` instance with an empty vector of cars.
- `insert_car(car: Car)`: Inserts a car into the parking lot. Checks for duplicates before insertion.
- `remove_car(make: &str, model: &str, car_plate: &str) -> Option<Car>`: Removes a car from the parking lot based on make, model, and car plate.
- `display_parking_lot()`: Displays the list of cars in the parking lot.

## Usage

1. **Create a Car:**
   ```rust
   let car_example = Car::new("Toyota", "Corolla", "FFH-672");
