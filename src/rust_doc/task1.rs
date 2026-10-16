use std::collections::HashMap;

fn solve_task1(list_int: &mut Vec<i32>) {
    list_int.sort();
    let len = list_int.len();

    let median: i32 = if len % 2 == 0 {
        list_int[len / 2]
    } else {
        let med1 = list_int[len / 2];
        let med2 = list_int[len / 2 + 1];
        (med1 + med2) / 2
    };

    println!("Median: {median}");

    let mut occurence: HashMap<i32, usize> = HashMap::new();
    for value in list_int {
        let tmp = occurence.entry(*value).or_insert(0);
        *tmp += 1;
    }

    let mut often = (0, 0);
    for (key, value) in &occurence {
        if often.1 < *value {
            often = (*key, *value);
        }
    }

    let value = often.0;
    let how_many = often.1;
    println!("Mode: {value} occoured {how_many} times.")
}

pub fn run_task() {
    let mut input = vec![3, 2, 4, 24, 3, 2, 3, 42, 3, 4, 234, 234, 23];
    solve_task1(&mut input);
}
