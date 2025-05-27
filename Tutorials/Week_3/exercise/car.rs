/**
 * Author: Dr. Peter Yau
 * Email: PeterCY.Yau@glasgow.ac.uk
 * 
 * Copyright 2025
 * School of Computing Science, University of Glasgow
 *  
 * Disclaimer:
 * Programmers should ensure the environment and settings are correct.
 * This code does not confirm 100% accuracy. Remember to change the filename and 
 * class name to match your specific implementation.
 *
 * Example: OOP illustrtion using structs and methods
 *
 */

struct Car {
    brand: String,
    fuel: u32, // fuel in liters
    running: bool,
}

impl Car {
    fn new(brand: &str, fuel: u32) -> Car {
        Car {
            brand: brand.to_string(),
            fuel,
            running: false,
        }
    }

    fn drive(&mut self, distance: u32) {
        if self.fuel == 0 {
            println!("{} can't drive: no fuel!", self.brand);
            return;
        }
        let needed_fuel = distance / 10; // suppose 1 liter per 10 km
        if self.fuel >= needed_fuel {
            self.fuel -= needed_fuel;
            self.running = true;
            println!(
                "{} drives {} km. Remaining fuel: {} liters.",
                self.brand, distance, self.fuel
            );
        } else {
            println!("{} can't drive that far: not enough fuel!", self.brand);
        }
    }

    fn refuel(&mut self, amount: u32) {
        self.fuel += amount;
        println!("{} refueled. Current fuel: {} liters.", self.brand, self.fuel);
    }

    fn stop(&mut self) {
        if self.running {
            self.running = false;
            println!("{} stopped.", self.brand);
        } else {
            println!("{} is already stopped.", self.brand);
        }
    }
}

fn main() {
    let mut my_car = Car::new("Toyota", 10);

    my_car.drive(50);    // uses 5 liters
    my_car.stop();
    my_car.drive(30);    // uses 3 liters
    my_car.refuel(10);   // refuel 10 liters
    my_car.drive(100);   // uses 10 liters
    my_car.stop();
}