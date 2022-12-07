use std::{fs::File, io::Read};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct City {
    pub x: f64,
    pub y: f64,
}

pub struct Christofides {
    // num of cities
    pub n: usize,
    // path
    pub path: Vec<usize>,
    // Shortest path length
    pub length: f64,
    // euler circuit
    pub circuit: Vec<usize>,
    // cities
    pub cities: Vec<City>,
    // n x n, pairwise distances between cities
    pub graph: Vec<Vec<f64>>,
    // n x n, adjacency matrix
    pub adj_list: Vec<Vec<usize>>,
    // List of odd nodes
    pub odds: Vec<usize>,
}
impl Christofides {
    pub fn new() -> Self {
        Christofides {
            n: 0,
            path: Vec::new(),
            length: 0.0,
            circuit: Vec::new(),
            cities: Vec::new(),
            graph: Vec::new(),
            adj_list: Vec::new(),
            odds: Vec::new(),
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
        self.adj_list = vec![vec![]; self.n];
    }

    pub fn get_distance(&self, c1: City, c2: City) -> f64 {
        let x = c1.x - c2.x;
        let y = c1.y - c2.y;
        (x * x + y * y).sqrt()
    }

    pub fn fill_matrix(&mut self) {
        for i in 0..self.n {
            for j in 0..self.n {
                self.graph[i][j] = self.get_distance(self.cities[i], self.cities[j]);
            }
        }
    }

    pub fn find_odds(&mut self) {
        for i in 0..self.n {
            if self.adj_list[i].len() % 2 == 1 {
                self.odds.push(i);
            }
        }
    }

    //Find perfect matching
    pub fn perfect_matching(&mut self) {
        // Find nodes with odd degrees in T to get subgraph O
        self.find_odds();

        // for each odd node
        for i in 0..self.odds.len() {
            // find the closest odd node
            let mut min = std::f64::MAX;
            let mut min_index = 0;
            for j in 0..self.odds.len() {
                if i != j {
                    let dist = self.graph[self.odds[i]][self.odds[j]];
                    if dist < min {
                        min = dist;
                        min_index = j;
                    }
                }
            }
            // add edge to adjacency list
            self.adj_list[self.odds[i]].push(self.odds[min_index]);
            self.adj_list[self.odds[min_index]].push(self.odds[i]);
        }
    }

    pub fn get_min_index(&self, key: &Vec<f64>, mst: &Vec<bool>) -> usize {
        let mut min = std::f64::MAX;
        let mut min_index = 0;
        for v in 0..self.n {
            if mst[v] == false && key[v] < min {
                min = key[v];
                min_index = v;
            }
        }
        min_index
    }

    pub fn find_mst(&mut self) {
        let mut key = vec![std::f64::MAX; self.n];
        let mut included = vec![false; self.n];
        let mut parent: Vec<i64> = vec![0; self.n];

        key[0] = 0.0;
        parent[0] = -1;

        for _ in 0..self.n - 1 {
            let u = self.get_min_index(&key, &included);
            included[u] = true;

            for v in 0..self.n {
                if self.graph[u][v] != 0.0 && included[v] == false && self.graph[u][v] < key[v] {
                    parent[v] = u as i64;
                    key[v] = self.graph[u][v];
                }
            }
        }

        for i in 1..self.n {
            let j = parent[i];

            if j != -1 {
                self.adj_list[i].push(j as usize);
                self.adj_list[j as usize].push(i);
            }
        }
    }

    pub fn euler_tour(&mut self, start: usize, path: &mut Vec<usize>) {
        // Create copy of adj_list
        let mut temp_list = self.adj_list.clone();

        // Create stack
        let mut stack: Vec<i64> = Vec::new();
        let mut pos = start;
        self.path.push(start);

        while !stack.is_empty() || !temp_list[pos].is_empty() {
            if temp_list[pos].is_empty() {
                path.push(pos);
                pos = stack.pop().unwrap() as usize;
            } else {
                stack.push(pos as i64);

                let neighbor = temp_list[pos].pop().unwrap();

                for i in 0..temp_list[neighbor].len() {
                    if temp_list[neighbor][i] == pos {
                        temp_list[neighbor].remove(i);
                        break;
                    }
                }

                pos = neighbor;
            }
        }
        self.path.push(pos);
    }

    pub fn make_hamiltonian(&mut self, path: &Vec<usize>) -> (f64, Vec<usize>) {
        let mut visited = vec![false; self.n];
        let mut hamiltonian_path = Vec::new();

        let mut path_cost = 0.0;

        let root = path[0];
        visited[root] = true;
        hamiltonian_path.push(path[0]);

        for i in 1..path.len() {
            let node = path[i - 1];
            let next_node = path[i];

            if visited[next_node] == false {
                visited[next_node] = true;
                path_cost += self.graph[node][next_node];
                hamiltonian_path.push(next_node);
            }
        }

        path_cost += self.graph[path[path.len() - 1]][root];

        (path_cost, hamiltonian_path)
    }

    pub fn find_best_path(&mut self) -> (f64, Vec<usize>) {
        let mut path: Vec<usize> = Vec::new();

        self.euler_tour(0, &mut path);
        let answer = self.make_hamiltonian(&path);

        answer
    }

    pub fn christofides(&mut self) -> (f64, Vec<usize>) {
        self.fill_matrix();
        self.find_mst();
        self.perfect_matching();
        let answer = self.find_best_path();

        answer
    }
}
