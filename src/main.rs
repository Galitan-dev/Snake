const HELLO_WORLD: &str = "Hello World!";

fn main() {
    for (i, char) in HELLO_WORLD.chars().enumerate() {
        let mut indentation = String::new();
        for _ in 0..i * 2 {
            indentation.push(' ');
        }

        println!("{} {}", indentation, char);
    }
}
