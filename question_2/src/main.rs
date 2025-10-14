fn main() {
    println!("{}", check_string_pair_sort("abcc", "cabc"));
}

fn check_string_pair(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut alphabet_array = [0; 26];

    for c in s1.to_lowercase().chars() {
        alphabet_array[c as usize - 97] += 1;
    }

    for c in s2.to_lowercase().chars() {
        alphabet_array[c as usize - 97] -= 1;
    }

    alphabet_array.iter().all(|&x| x == 0)
}

fn check_string_pair_sort(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let (mut s1_vec, mut s2_vec) = (
        s1.chars().collect::<Vec<char>>(),
        s2.chars().collect::<Vec<char>>(),
    );

    s1_vec.sort();
    s2_vec.sort();

    let (s1, s2) = (
        s1_vec.iter().collect::<String>(),
        s2_vec.iter().collect::<String>()
    );

    s1 == s2
}
