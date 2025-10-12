use std::collections::HashSet;

fn main() {
    // println!("{}", is_unique_hash_set("aabc"));
    // println!("{}", is_unique_hash_set("abc"));
    println!("{}", is_unique_sorting("aabc"));
    println!("{}", is_unique_sorting("abc"));
}

fn is_unique_hash_set(string: &str) -> bool{
    let set = string.chars().collect::<HashSet<char>>();

    set.len() == string.len()
}

fn is_unique_loops(string: &str) -> bool {
    for i in 0..string.len() {
        for j in 0..string.len() {
            if string.chars().nth(i).unwrap() == string.chars().nth(j).unwrap() {
                return false;
            }
        }
    }

    true
}

fn is_unique_sorting(string: &str) -> bool {
    let mut chars = string.chars().collect::<Vec<char>>();

    chars.sort();

    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            return false;
        }
    }

    true
}