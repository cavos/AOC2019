
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

mod interpreter;

fn main() {
    println!("Hello, Advent of Code 2019!");
    day01::solve("./data/day01.txt");
    day01::solve_pt2("./data/day01.txt");
    day02::solve("./data/day02.txt");
    day02::solve_pt2("./data/day02.txt", 19690720);
    day03::solve("./data/day03.txt");
    day04::solve(168630,718098);
    day05::solve("./data/day05.txt");
    day05::solve_pt2("./data/day05.txt");
    day06::solve("./data/day06.txt");
    day06::solve_pt2("./data/day06.txt");
}
