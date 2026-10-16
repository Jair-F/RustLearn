fn sovle_task2(word: &str) {
    let new = word.to_string();

    if word.len() < 1 {
        return;
    }

    let chars: Vec<char> = new.to_lowercase().chars().collect();

    let mut ret = word[1..].to_string() + "-";
    let insert = match chars[0] {
        'a' | 'e' | 'i' | 'o' | 'u' => Some(word.chars().next().unwrap()),
        _ => None,
    };

    if let Some(c) = insert {
        ret.insert(0, c);
        ret += "h";
    } else {
        ret += &word[0..1];
    }
    ret += "ay";

    println!("task2: {ret}")
}

pub fn run_task() {
    let word = String::from("first"); // Needs to become irst-fay
    // let word = String::from("apple"); // Needs to become apple-hay
    sovle_task2(&word);
}
