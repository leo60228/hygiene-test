use hygiene_test::example;

fn main() {
    let var = 5;
    example!();
    println!("user: {}", var);
}
