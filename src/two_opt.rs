use std::{fs::File, io::Read};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

pub struct TwoOpt {
    // num of cities
    pub n: usize,
    // path
    pub path: Vec<usize>,
    // Shortest path length
    pub length: f64,
    // euler circuit
    pub cities: Vec<City>,
    // n x n, pairwise distances between cities
    pub graph: Vec<Vec<f64>>,
}
impl TwoOpt {
    pub fn new() -> Self {
        TwoOpt {
            n: 0,
            path: Vec::new(),
            length: 0.0,
            cities: Vec::new(),
            graph: Vec::new(),
        }
    }

    pub fn read_file(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let lines = contents.lines();

        for line in lines {
            let mut iter = line.split_whitespace();
            let _ = iter.next().unwrap().parse::<usize>().unwrap();
            let x = iter.next().unwrap().parse::<f64>().unwrap();
            let y = iter.next().unwrap().parse::<f64>().unwrap();
            self.cities.push(City { x, y });
        }

        self.n = self.cities.len();
        self.graph = vec![vec![0.0; self.n]; self.n];
    }

    pub fn read_init_path(&mut self, path: Vec<usize>) {
        self.path = path;
    }

    pub fn get_distance(&self, c1: City, c2: City) -> f64 {
        ((c1.x - c2.x).powi(2) + (c1.y - c2.y).powi(2)).sqrt()
    }

    pub fn fill_matrix(&mut self) {
        for i in 0..self.n {
            for j in 0..self.n {
                self.graph[i][j] = self.get_distance(self.cities[i], self.cities[j]);
            }
        }
    }

    pub fn get_path_length(&self, path: &Vec<usize>) -> f64 {
        let mut length = 0.0;
        for i in 0..self.n {
            length += self.graph[path[i]][path[(i + 1) % self.n]];
        }
        length
    }

    pub fn two_opt(&mut self) {
        self.fill_matrix();
        let mut best_path = self.path.clone();
        let mut best_length = self.get_path_length(&self.path);
        // println!("best_length: {:?}", best_length);

        let mut improved = true;
        while improved {
            improved = false;
            for i in 0..self.n {
                for j in i + 1..self.n {
                    let mut new_path = best_path.clone();
                    new_path[i..=j].reverse();
                    let new_length = self.get_path_length(&new_path);
                    // println!("best_length: {}, new_length: {}", best_length, new_length);
                    if new_length < best_length {
                        best_path = new_path;
                        best_length = new_length;
                        improved = true;
                    }
                }
            }
        }

        self.path = best_path;
        self.length = best_length;
    }
}
