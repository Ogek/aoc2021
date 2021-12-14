use std::time::Instant;

mod day14;

fn main() {
    let now = Instant::now();
    let p1_ans = day14::p1(include_str!("../inputs/day14.txt"));
    let p1_took = now.elapsed();

    let now = Instant::now();
    let p2_ans = day14::p2(include_str!("../inputs/day14.txt"));
    let p2_took = now.elapsed();

    println!("P1 Solution: {:?}", p1_ans);
    println!("P1 Took: {:?}", p1_took);
    println!("P2 Solution: {:?}", p2_ans);
    println!("P2 Took: {:?}", p2_took);
}
