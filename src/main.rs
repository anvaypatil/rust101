// mod leetcode;
fn main() {
    // leetcode::arrays::l1_two_sum::execute();
    partial();
}

fn partial() {
    let m = 10;
    match m {
        1..=3 => println!("a"),
        10 | 12 | 13 => println!("c"),
        15..=30 => println!("b"),
        _ => println!("default")
    }
}

