
fn process_telemetry(readings: Vec<f64>) {
    let sum: f64 = readings.iter().sum();
    println!("Sum: {sum}");
}

fn process_borrowed(readings: &[f64]) {
    let sum: f64 = readings.iter().sum();
    println!("Sum: {sum}");
}

pub fn test_start() {
    let data = vec![1.0, 2.0, 3.0];

    process_borrowed(&data);
    println!("{:?}", data);
    process_telemetry(data);
    // println!("{:?}", data);
}