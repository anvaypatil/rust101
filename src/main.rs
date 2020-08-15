// mod leetcode;
fn main() {
    // leetcode::arrays::l1_two_sum::execute();
    partial();
}

fn partial() {
    let v = vec![4, 5, 6, 7, 4, 5, 6, 4, 4, 3, 3, 5, 5, 3, 4, 5, 5, 6, 3];
    for i in &v {
        let r: i32 = count(&v, i);
        println!("{} -> {}", , String::from(r) );
    }
}

fn count(v: &Vec<i32>, val: &i32) -> i32 {
    v.into_iter().filter(|&&x| x == *val).count() as i32
}
