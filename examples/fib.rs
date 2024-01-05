fn main() {
    let n: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", fibonacci::iterative(n));
}
