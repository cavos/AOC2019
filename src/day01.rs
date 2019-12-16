use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input_data: &str) {
    let buffered = BufReader::new(File::open(input_data).unwrap());
    let result = buffered
        .lines()
        .map(|l| l.unwrap())
        .fold(0i32, |fuel_sum, mass| fuel_sum + (mass.parse::<i32>().unwrap()/3) - 2);
    println!("Day 01.1: Sum of fuel requirements is {:} units.", result);
}

struct FuelRequirement {
    mass : i32,
}

impl FuelRequirement {
    fn new(mass : i32) -> FuelRequirement {
        FuelRequirement {mass : mass}
    }
}

impl Iterator for FuelRequirement {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let fuel_req = (self.mass / 3) - 2;
        if fuel_req > 0 {
            self.mass = fuel_req;
            Some(fuel_req)
        } else {
            None
        }
    }
}

pub fn solve_pt2(input_data: &str) {
    let buffered = BufReader::new(File::open(input_data).unwrap());
    let total_fuel_required = buffered
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .fold(0i32, |fuel_sum, mass| {
            let fuel_req : i32 = FuelRequirement::new(mass).sum();
            fuel_sum + fuel_req
            });

    println!("Day 01.2: Sum of fuel requirements is {:} units.", total_fuel_required);
}
