fn main() {
    let result = urlify("John Doe");
    println!("{}", result);
}

fn urlify(string: &str) -> String {
    string.chars()
        .map(|char| {
            if char == ' ' {
                "%20".to_string()
            } else {
                char.to_string()
            }
        })
        .collect()
}
