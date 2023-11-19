fn main() {
    let sum = (1..=100).fold(0, std::ops::Add::add);
    println!("{sum}");
}
