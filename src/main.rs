mod christofides;
mod two_opt;

use christofides::Christofides;
use two_opt::TwoOpt;

fn main() {
    // "data/tsp_example_1.txt"
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut cf = Christofides::new();
    cf.read_file(filename);
    let answer = cf.christofides();

    println!("path: {:?}", answer.1);

    let mut to = TwoOpt::new();
    to.read_file(filename);
    to.read_init_path(answer.1);
    to.two_opt();

    println!("path cost: {}", to.get_path_length(&to.path));
    println!("path: {:?}", to.path);
}
