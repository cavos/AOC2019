use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::btree_map::{BTreeMap, Entry};
use std::collections::VecDeque;

struct SpaceObject {
    orbits: Option<String>,
    on_orbit: Vec<String>
}

impl SpaceObject {
    pub fn new( on_orbit: &String) -> Self {
        SpaceObject {
            orbits: None,
            on_orbit: vec![on_orbit.clone()]
        }
    }

    pub fn for_orbit(orbits: &String) -> Self {
        SpaceObject {
            orbits: Some(orbits.clone()),
            on_orbit: Vec::new()
        }
    }

    pub fn add_on_orbit(&mut self, object_id: &String) {
        self.on_orbit.push(object_id.clone());
    }
}

pub fn solve(input_file: &str) {
    let orbit_treemap = make_orbit_treemap(input_file);
    

    let mut orbit_count = 0u32;
    let com_object_id = String::from("COM"); 
    let mut to_visit : VecDeque<&String> = VecDeque::new();
    to_visit.push_back(&com_object_id);
    let mut distances : BTreeMap<String, u32> = BTreeMap::new();
    while to_visit.len() > 0 {
        let object_id = to_visit.pop_front().unwrap();
        let space_object = orbit_treemap.get(object_id).expect("UNKNOWN OBJECT");
        
        if object_id == &com_object_id {
            distances.insert(object_id.clone(), 0);
        } else {
            assert!(space_object.orbits.is_some());
            let com_dist = distances.get(space_object.orbits.as_ref().unwrap()).expect("NOT KNOWN DISTANCE") + 1;
            orbit_count += com_dist;
            distances.insert(object_id.clone(), com_dist);
        }

        for obj in &space_object.on_orbit {
            to_visit.push_back(obj);
        }
    }

    println!("Day 06.1: Orbit count checksum is {}", orbit_count);
}

pub fn solve_pt2(input_file: &str) {
    let orbit_treemap = make_orbit_treemap(input_file);

    let mut transfer_count = 0u32;
    let santa = String::from("SAN");
    let mut to_visit : VecDeque<String> = VecDeque::new();
    let parent = orbit_treemap.get("YOU").expect("UNKNOWN ME").orbits.as_ref().expect("Parent missing!");
    to_visit.push_front(parent.clone());
    let mut distances : BTreeMap<String, u32> = BTreeMap::new();
    distances.insert(parent.clone(), 0);
    while to_visit.len() > 0 {
        let object_id = to_visit.pop_front().unwrap();

        // check if santa is on current object orbit
        let object = orbit_treemap.get(&object_id).expect("UNKNOWN OBJECT");
        let object_distance = *distances.get(&object_id).expect("Unknown distance!");
        if object.on_orbit.iter().find(|&x| x == &santa).is_some() {
            transfer_count = object_distance;
            break;
        }

        if object_id == "COM" {
            continue;
        }

        //add parent to search list
        let parent_id = object.orbits.as_ref().expect("Missing parent");
        process_path_node(parent_id, object_distance+1, &mut distances, &mut to_visit);

        //and all of children!
        for child in &object.on_orbit {
            process_path_node(child, object_distance+1, &mut distances, &mut to_visit);
        }
    }

    println!("Day 06.2: Required transfer count: {}", transfer_count);
}

fn process_path_node(node_id: &String, distance: u32, distances: &mut BTreeMap<String, u32>, to_visit: &mut VecDeque<String>) {
    let child_dist = distances.get(node_id);
    if child_dist.is_none() || child_dist.unwrap() > &distance {
        *distances.entry(node_id.clone()).or_insert(0) = distance;
        to_visit.push_back(node_id.clone());
    }
}

fn make_orbit_treemap(input_file: &str) -> BTreeMap<String, SpaceObject> {
    let orbit_map_raw = read_orbit_map(input_file);
    
    let mut orbit_treemap: BTreeMap<String, SpaceObject> = BTreeMap::new();
    for (object, orbits) in orbit_map_raw {
        match orbit_treemap.entry(object.clone()) {
            Entry::Vacant(v) =>{ v.insert(SpaceObject::new(&orbits)); },
            Entry::Occupied(mut o) => o.get_mut().add_on_orbit(&orbits)
        }

        match orbit_treemap.entry(orbits.clone()) {
            Entry::Vacant(v) => { v.insert(SpaceObject::for_orbit(&object)); },
            Entry::Occupied(mut o) => o.get_mut().orbits = Some(object.clone())
        }
    }
    orbit_treemap
}

fn read_orbit_map(input_file: &str) -> Vec<(String, String)> {
    let buffered = BufReader::new(File::open(input_file).unwrap());
    buffered.lines().map(|x| x.unwrap())
        .map(|x| x.split(')').map(|z: &str| String::from(z)).collect())
        .map(|x: Vec<String>| (x[0].clone(), x[1].clone()))
        .collect()
}