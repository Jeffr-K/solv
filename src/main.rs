
mod leet;
use leet::arrays::insert::*;

fn main() {
    let vec = vec![1,2,3,4,5];
    let result = find_max_consecutive_ones(vec);
    println!("result is {}", result);
}
