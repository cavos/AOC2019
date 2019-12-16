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
    let me = String::from("YOU");
    let mut to_visit : VecDeque<&String> = VecDeque::new();
    to_visit.push_front(&me);
    let mut visited = VecDeque::new();
    let mut distances : BTreeMap<String, u32> = BTreeMap::new();
    while to_visit.len() > 0 {
        let object_id = to_visit.pop_front().unwrap();
        visited.push_back(object_id);
        println!("visiting {} at distance {:?}", object_id, distances.get(object_id));
        if object_id == "COM" {
            continue;
        }

        let parent_id = orbit_treemap.get(object_id).expect("UNKNOWN ME").orbits.as_ref().expect("MUST HAVE 'PARENT'");
        let parent = orbit_treemap.get(parent_id).unwrap();
        if parent.on_orbit.iter().find(|&x| x == &santa).is_some() {
            println!("ob {}, parent {}, orbit {:?}", object_id, parent_id, parent.on_orbit);
            transfer_count = *distances.get(parent_id).expect("NOT COMPUTED DISTANCE");
            break;
        } else {
            if object_id == &me {
                distances.insert(parent_id.clone(), 0);
                for o in &parent.on_orbit {
                    distances.insert(o.clone(), 0);
                }
            } else {
                let dist = distances.get(object_id).expect("!!!") + 1;
                distances.insert(parent_id.clone(), dist);
                for o in &parent.on_orbit {
                    distances.insert(o.clone(), dist);
                }
            }
            
            for o in &parent.on_orbit {
                if visited.iter().find(|&x| x == &o).is_none() {
                to_visit.push_front(o);
            }
            if visited.iter().find(|&x| x == &parent_id).is_none() {
                to_visit.push_front(parent_id);
            }
            }
        }
    }

    println!("Day 06.2: Required transfer count: {}", transfer_count);
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