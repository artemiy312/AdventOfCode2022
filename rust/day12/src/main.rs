use std::{collections::{LinkedList, HashMap}};

type XY = (i64, i64);

struct Heightmap {
    map: Vec<Vec<char>>,
    elevations: HashMap<char, char>,
}

impl Heightmap {
    fn new(map: String) -> Self {
        let heightmap = map
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let mut elevations = HashMap::new();
        elevations.insert('S', 'a');
        elevations.insert('z', 'E');
        let all_chars = ('a'..='z').collect::<Vec<char>>();
        for (i, char) in all_chars.iter().enumerate().skip(1) {
            elevations.insert(all_chars[i - 1], char.clone());
        }

        Heightmap { map: heightmap, elevations }
    }

    fn is_possible_elevation(&self, from: XY, to: XY) -> bool {
        let (x, y) = from;
        let (xn, yn) = to;

        if xn < 0 || yn < 0 || yn >= self.map.len() as i64 {
            return false;
        }

        if xn >= self.map[yn as usize].len() as i64 {
            return false;
        }

        let chr = self.map[y as usize][x as usize];
        let chrn = self.map[yn as usize][xn as usize];

        chr == chrn || self.elevations[&chr] == chrn
    }

    fn find_height(&self, name: &char) -> Option<XY> {
        for (y, line) in self.map.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if char == name {
                    return Some((x as i64, y as i64));
                }
            }
        }
        return None;
    }
}

static sides: [XY; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
// struct BFS {
//     frontier: LinkedList<XY>,
//     visited: HashMap<XY, XY>,
//     heightmap: Heightmap
// }

// impl BFS {
//     fn new(heightmap: Heightmap) -> Self {
//         BFS {
//             frontier: LinkedList::new(),
//             visited: HashMap::new(),
//             heightmap: heightmap,
//         }
//     }
// }

// impl Iterator for BFS {
//     type Item = XY;

//     fn next(&mut self) -> Option<Self::Item> {
//         return None
//     }
// }




fn main() {
    let name = std::env::args().nth(1).expect("path is missed");
    // let name = "../../inputs/12.txt";
    let content = std::fs::read_to_string(name).expect("can't read file");

    let heightmap = Heightmap::new(content);
    let start_xy = heightmap.find_height(&'S').unwrap();
    let end_xy = heightmap.find_height(&'E').unwrap();

    let mut frontier = LinkedList::new();
    let mut path_to_exit = HashMap::new();
    frontier.push_back(start_xy);

    let mut populate_path_to_exit = || {
        while let Some(xy) = frontier.pop_front() {
            let (x, y) = xy;
            if xy == end_xy {
                return Some((x, y));
            }
            for (x_offset, y_offset) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (xn, yn) = (x + x_offset, y + y_offset);
                if path_to_exit.contains_key(&(xn, yn))  { continue; }
                if heightmap.is_possible_elevation(xy, (xn, yn)) {
                    path_to_exit.insert((xn, yn), (x, y));
                    frontier.push_back((xn, yn));
                }
            }
        }
        return None
    };
    let mut step_count = 0;
    let mut xy = populate_path_to_exit().unwrap();
    while let Some(nxy) = path_to_exit.get(&xy) {
        step_count += 1;
        if *nxy == start_xy {
            break;
        }
        xy = *nxy;
    }
    println!("{}", step_count);
}
