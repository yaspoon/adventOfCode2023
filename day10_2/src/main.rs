use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    sym: char,
    row: usize,
    col: usize,
}

#[derive(PartialEq,Eq,Hash,Clone,Copy,Debug)]
enum Direction {
    North = 0,
    South,
    East,
    West,
}

impl Node {
    fn new(sym: char, row: usize, col: usize) -> Node {
        Node {sym, row, col}
    }
}

fn check_node(start: &Node, node: &Node) -> bool {
    let direction = get_direction(&start, &node);
    let dir_set = HashSet::from([direction]);
    let pipe_dirs = get_directions_for_pipe(&node);
    if pipe_dirs.difference(&dir_set).count() == 1 {
        true
    } else {
        false
    }
}

fn find_first_node(start: &Node, map: &Vec<Vec<char>>) -> Node {
    let row_len = map[0].len();
    let row = start.row;
    let col = start.col;

    if col + 1 < row_len {
        let tmp = Node::new(map[row][col+1], row, col+1);
        if check_node(start, &tmp) {
            return tmp;
        }
    }

    if col > 0 {
        let tmp = Node::new(map[row][col-1], row, col-1);
        if check_node(start, &tmp) {
            return tmp;
        }
    }

    if row > 0 {
        let tmp = Node::new(map[row-1][col], row-1, col);
        if check_node(start, &tmp) {
            return tmp;
        }
    }

    if row < map.len() {
        let tmp = Node::new(map[row+1][col], row+1, col);
        if check_node(start, &tmp) {
            return tmp;
        }
    }

    panic!("No valid first node found");

    //Node::new('-', start.row, start.col + 1)
    //Node::new('J', start.row, start.col + 1)
}

fn get_direction(from: &Node, to: &Node) -> Direction {
    match to.row.checked_sub(from.row) {
        Some(t) => {
            if t > 0 {
                return Direction::North
            }
        },
        None => return Direction::South
    }

    match to.col.checked_sub(from.col) {
        Some(t) => {
            if t > 0 {
                return Direction::West
            } else {
                panic!("Wtf happened");
            }
        },
        None => return Direction::East
    }

    /*
    let left_right = to.col - from.col;

    if up_down != 0 {
        if up_down > 0 {
            Direction::North
        } else {
            Direction::South
        }
    } else if left_right != 0 {
        if left_right > 0 {
            Direction::East
        } else {
            Direction::West
        }
    } else {
        panic!("Wtf happened up_down:{} left_right:{}", up_down, left_right);
    }
    */
}

fn get_directions_for_pipe(node: &Node) -> HashSet<Direction> {
    let directions = match node.sym {
        '|' => [Direction::North, Direction::South],
        '-' => [Direction::West, Direction::East],
        'L' => [Direction::North, Direction::East],
        'J' => [Direction::North, Direction::West],
        '7' => [Direction::South, Direction::West],
        'F' => [Direction::South, Direction::East],
        _ => panic!("No directions for {}", node.sym),
    };

    HashSet::from(directions)
}

fn get_next(cur: &Node, direction: &Direction, map: &Vec<Vec<char>>) -> Node {
    let directions = get_directions_for_pipe(cur);
    let from_dir = HashSet::from([*direction]);

    //println!("cur:{:?} directions:{:?} from_dir:{:?}", cur, directions, from_dir);

    let to_dir: Vec<Direction> = directions.difference(&from_dir).copied().collect();
    if to_dir.len() != 1 {
        panic!("unexpected to_dir{:?}", to_dir);
    }

    let mut row = cur.row;
    let mut col = cur.col;

    match to_dir[0] {
        Direction::North => {
            row -= 1;
        },
        Direction::South => {
            row += 1;
        }
        Direction::East => {
            col += 1;
        }
        Direction::West=> {
            col -= 1;
        }
    }

    Node::new(map[row][col], row, col)
}

fn is_adjacent(row: usize, col: usize, done: &HashSet<(usize,usize)>, on_path: &HashSet<(usize,usize)>) -> bool {
    if !on_path.contains(&(row, col)) && !done.contains(&(row, col)) {
        true
    } else {
        false
    }
}

fn find_adjacents(height: usize, width: usize, row: usize, col: usize, blob: &mut Vec<(usize, usize)>, done: &mut HashSet<(usize, usize)>, on_path: &HashSet<(usize, usize)>) {
    if !done.contains(&(row, col)) {
        done.insert((row, col));
        blob.push((row, col));
        //up
        if row > 0 {
            if is_adjacent(row - 1, col, done, on_path) {
                find_adjacents(height, width, row - 1, col, blob, done, on_path);
            }
        }

        //down
        if row < (height - 1) {
            if is_adjacent(row + 1, col, done, on_path) {
                find_adjacents(height, width, row + 1, col, blob, done, on_path);
            }
        }

        //left
        if col > 0 {
            if is_adjacent(row, col - 1, done, on_path) {
                find_adjacents(height, width, row, col - 1, blob, done, on_path);
            }
        }

        //right
        if col < (width - 1) {
            if is_adjacent(row, col + 1, done, on_path) {
                find_adjacents(height, width, row, col + 1, blob, done, on_path);
            }
        }
    }
}

fn flood_fill(width: usize, height: usize, map: &Vec<Vec<char>>, path: &Vec<Node>) -> Vec<Vec<(usize,usize)>> {
    let mut blobs: Vec<Vec<(usize,usize)>> = Vec::new();
    let mut on_path: HashSet<(usize, usize)> = HashSet::new();

    for n in path.iter() {
        on_path.insert((n.row, n.col));
    }

    let mut done: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            //Make sure this node isn't part of the path
            if !on_path.contains(&(y,x)) {
                if !done.contains(&(y,x)) { //We haven't seen this before
                    let mut blob = Vec::new();
                    find_adjacents(height, width, y, x, &mut blob, &mut done, &on_path);
                    blobs.push(blob);
                }
            }
        }
    }

    blobs
}

fn print_map(map: &Vec<Vec<char>>) {
    for l in map.iter() {
        for c in l.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

fn overlay_blobs_onto_map(map: &Vec<Vec<char>>, blobs: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();

    for l in map {
        new_map.push(l.clone());
    }

    let mut asdf = 65;
    for b in blobs.iter() {
        let c = char::from_u32(asdf).unwrap();
        asdf += 1;
        for (row, col) in b.iter() {
            new_map[*row][*col] = c;
        }
    }

    new_map
}

fn main() {
    let path = Path::new("sample_input");
    //let path = Path::new("input");

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

    let map: Vec<Vec<char>> = lines.into_iter().map(|o| o.chars().collect::<Vec<char>>()).collect();

    let mut start_row = 0;
    let mut start_col = 0;
    let mut found = false;
    for (r,l) in map.iter().enumerate() {
        match l.iter().position(|o| *o == 'S') {
            Some(c) => {
                start_row = r;
                start_col = c;
                found = true;
                break;
            }
            None => (),
        }
    }
    //println!("row:{}, col:{} found:{}", start_row, start_col, found);
    if !found {
        panic!("failed to find start");
    }

    let start = Node::new('S', start_row, start_col);
    let first = find_first_node(&start, &map);

    
    let width = map[0].len();
    let height = map.len();
    let mut path: Vec<Node> = vec![start,first];
    let mut direction = get_direction(&path[0], &path[1]);
    let mut cur_index = 1;
    while path[cur_index].sym != 'S' {
        let next = get_next(&path[cur_index], &direction, &map);
        direction = get_direction(&path[cur_index], &next);
        path.push(next);
        cur_index += 1;
    }

    //println!("path:{:?}", path);
    let blobs = flood_fill(width, height, &map, &path);
    println!("blobs:{:?}", blobs);

    let new_map = overlay_blobs_onto_map(&map, &blobs);

    print_map(&map);
    print_map(&new_map);

}
