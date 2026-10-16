fn sovle_task2(word: &str) {
    if word.is_empty() {
        return;
    }

    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    let rest = chars.as_str();

    let ret = match first_char.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{word}-hay"),
        _ => format!("{rest}-{first_char}ay"),
    };

    println!("task2: {ret}")
}

pub fn run_task() {
    // let word = String::from("first"); // Needs to become irst-fay
    let word = String::from("apple"); // Needs to become apple-hay
    sovle_task2(&word);
}
