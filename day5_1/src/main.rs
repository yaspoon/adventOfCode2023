use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;

fn parse_seeds(line: &str) -> Vec<usize> {
    line.split(": ").last().unwrap().split(" ").map(|o| o.parse::<usize>().unwrap()).collect()
}

fn is_empty_line(line: &str) -> bool {
    match line {
        "" => true,
        _ => false,
    }
}

fn parse_input_for_maps_list<'a>(lines: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut maps = Vec::new();
    let mut cur_map = Vec::new();
    for line in lines[2..].into_iter() {
        if is_empty_line(line) {
            maps.push(cur_map);
            cur_map = Vec::new();
        } else {
            cur_map.push(*line);
        }
    }
    maps.push(cur_map);
    maps
}

fn get_source_destination_names(line: &str) -> (&str, &str){
    let parts: Vec<&str> = line.split(" map:").next().unwrap().split("-to-").collect();
    (parts[0], parts[1])
}

#[derive(Debug)]
struct Map<'a> {
    source: &'a str,
    dest: &'a str,
    mappings: Vec<(usize, usize, usize)>,
}

impl<'a> Map<'a> {
    fn new(source: &'a str, dest: &'a str, mappings: Vec<(usize, usize, usize)>) -> Map<'a> {
        Map { source, dest, mappings }
    }

    fn find_mapping(self: &Self, lookup: usize) -> Option<(usize, usize)> {
        for mapping in self.mappings.iter() {
            let dest = mapping.0;
            let source = mapping.1;
            let count = mapping.2;
            let start = source;
            let end = start + count;

            if lookup >= start && lookup < end {
                //println!("Found mapping, lookup:{} start:{} end:{}", lookup, start, end);
                return Some((start, dest));
            }
        }

        None
    }

    fn lookup(self: &Self, lookup: usize) -> usize {
        match self.find_mapping(lookup) {
            Some((source, dest)) => {
                //println!("dest:{}, source:{}, lookup:{}", dest, source, lookup);
                dest + (lookup - source)
            },
            None => lookup,
        }
    }
}

fn map_entry(line: &str) -> (usize, usize, usize) {
    let parts: Vec<usize> = line.split(" ").map(|o| o.parse::<usize>().unwrap()).collect();
    if let [dest, source, count] = parts[0..3] {
        return (dest, source, count);
    } else {
        panic!("Not enough elements in line!");
    }
}

fn create_map_from_lines(lines: Vec<&str>) -> (&str, Map) {
    let (source, dest) = get_source_destination_names(lines[0]);
    //println!("source:{} dest:{}", source, dest);
    let mappings = lines[1..].into_iter().map(|o| map_entry(o)).collect::<Vec<(usize, usize, usize)>>();
    (source, Map::new(source, dest, mappings))
}

fn create_maps_from_list(maps_list: Vec<Vec<&str>>) -> HashMap<&str, Map> {
    maps_list.into_iter().map(|o| create_map_from_lines(o)).collect()
}

fn get_location(seed: usize, maps: & HashMap<&str, Map>) -> usize {
    let mut map_name = "seed";
    let mut lookup = seed;
    while map_name != "location" {
        println!("{} {},", map_name, lookup);
        let map = maps.get(map_name).unwrap();
        let next_lookup = map.lookup(lookup);
        //println!("map_name:{} lookup:{}->next_lookup:{}", map_name, lookup, next_lookup);
        map_name = map.dest;
        lookup = next_lookup;
    }
    println!();
    lookup
}

fn main() {
    //let path = Path::new("sample_input");
    let path = Path::new("input");

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file:{:?}", path),
    };

    let mut input: String = String::new();
    match file.read_to_string(&mut input) {
        Ok(_) => (),
        Err(e) => panic!("Failed to read file into string"),
    }

    let lines: Vec<&str> = input.lines().collect();
    //println!("lines.len:{}", lines.len());

    let seeds: Vec<usize> = parse_seeds(lines[0]);
    //println!("seeds:{:?}", seeds);

    let maps_list = parse_input_for_maps_list(lines);
    //println!("maps_list:{:?}", maps_list);

    let maps = create_maps_from_list(maps_list);
    //println!("maps:{:?}", maps);
    //println!("maps:{:?}", maps.get("seed").unwrap());

    let mut lowest = usize::MAX;
    for seed in seeds {
        let location = get_location(seed, &maps);
        if location < lowest {
            lowest = location;
        }
        //println!("seed:{} location:{}", seed, location);
    }

    println!("lowest:{}", lowest);
}
