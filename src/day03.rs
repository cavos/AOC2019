use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct Coord {
    dir : char,
    offset : u16
}

#[derive(Eq, Debug, Clone)]
struct Point {
    x : i32,
    y : i32
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut c = self.x.cmp(&other.x);
        if c == Ordering::Equal {
            c = self.y.cmp(&other.y)
        }
        c
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn distance_to(&self, other: &Self) -> u32 {
        let distance = (self.x - other.x).abs() as u32;
        distance + (self.y - other.y).abs() as u32
    }

    pub fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }
}

pub fn solve(puzzle_input : &str) {
    let (path_a, path_b) = read_input(puzzle_input);

    let wire_a = create_wire(path_a);
    let wire_b = create_wire(path_b);   

    let mut distance = u32::max_value();
    let mut step_count = u32::max_value();
    let central_point = Point{x: 0, y: 0};
    for line_a in wire_a.windows(2) {
        for line_b in wire_b.windows(2) {
            let intersection = get_intersection_point(&line_a, &line_b);
            if intersection.is_some() {
                let intersection = intersection.unwrap();
                let dist = central_point.distance_to(&intersection);
                if dist < distance {
                    distance = dist;
                }

                let mut required_steps = get_path_length_to(&wire_a, &intersection);
                required_steps += get_path_length_to(&wire_b, &intersection);
                if required_steps < step_count {
                    step_count = required_steps;
                }
            }
        }        
    }
  
    println!("Day 03.1: distance to closest intersection is {}", distance);
    println!("Day 03.2: {} steps required to reach intersection", step_count);
}

fn get_path_length_to(wire :&[Point], point: &Point) -> u32 {
    let mut step_count = 0u32;
    for line in wire.windows(2) {
        if is_point_on_line(&point, line) {
            step_count += point.distance_to(&line[0]);
            break;
        } else {
            step_count += line[0].distance_to(&line[1]);
        }
    }

    step_count
}

fn get_intersection_point(line_a: &[Point], line_b: &[Point]) -> Option<Point> {
    let horizontal_a = line_a[0].y == line_a[1].y;
    let horizontal_b = line_b[0].y == line_b[1].y;
    if horizontal_a != horizontal_b {
        let point: Point;
        if horizontal_b {
            point = Point{x: line_a[0].x, y: line_b[0].y};
        } else {
            point = Point{x: line_b[0].x, y: line_a[0].y};
        }
        if point != Point::new(0, 0) 
            && is_point_on_line(&point, line_a) && is_point_on_line(&point, line_b) {
            return Some(point)
        }
    }
    None
}

fn is_point_on_line(point: &Point, line: &[Point]) -> bool {
    let x1;
    let x2;
    if line[0].x > line[1].x {
        x1 = line[1].x;
        x2 = line[0].x;
    } else {
        x1 = line[0].x;
        x2 = line[1].x;
    }

    let y1;
    let y2;
    if line[0].y > line[1].y {
        y1 = line[1].y;
        y2 = line[0].y;
    } else {
        y1 = line[0].y;
        y2 = line[1].y;
    }

    x1 <= point.x && point.x <= x2 && y1 <= point.y && point.y <= y2
}

fn create_wire(path: Vec<Coord>) -> Vec<Point> {
    let mut cur_pos = Point{ x: 0, y: 0};
    let mut wire = vec![cur_pos.clone()];

    for coord in path {
        update_pos(&coord, &mut cur_pos);
        wire.push(cur_pos.clone())
    }

    wire
}

fn update_pos(coord: &Coord, point: &mut Point) {
    match coord.dir {
        'L' => point.x -= coord.offset as i32,
        'R' => point.x += coord.offset as i32,
        'U' => point.y += coord.offset as i32,
        'D' => point.y -= coord.offset as i32,
        _ => panic!("Unknown direcion!")
    }
}

fn read_input(input_file : &str) -> (Vec<Coord>, Vec<Coord>) {
    let buffered = BufReader::new(File::open(input_file).unwrap());
    let mut lines = buffered.lines();
    let path_a = parse_path(&lines.next().unwrap().unwrap());
    let path_b = parse_path(&lines.next().unwrap().unwrap());

    (path_a, path_b)
}

fn parse_path(line : &String) -> Vec<Coord> {
    let mut path = Vec::new();
    for coord in line.split(',') {
        let (row, pos) = coord.split_at(1);
        path.push(Coord { dir: row.chars().nth(0).unwrap(), offset: pos.parse::<u16>().unwrap()})
    }

    path
}