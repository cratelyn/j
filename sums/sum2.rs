fn main() {
    println!(
        "{}",
        (1..=100).fold(0, |cum, cur| cum + cur)
    );
}
