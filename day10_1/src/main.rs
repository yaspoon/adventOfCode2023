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
    println!("row:{}, col:{} found:{}", start_row, start_col, found);
    if !found {
        panic!("failed to find start");
    }

    let start = Node::new('S', start_row, start_col);
    let first = find_first_node(&start, &map);

    println!("Found first node:{:?}", first);

    
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

    println!("path:{:?}", path);

    let count: f64 = path.len() as f64 - 2.0;
    let mid = (count / 2.0).ceil();
    println!("mid:{}", mid);
}
