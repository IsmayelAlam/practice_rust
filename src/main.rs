use std::io;

fn main() {
    println!("Input your current weight: ");
    let mut current_weight = String::new();
    io::stdin().read_line(&mut current_weight).unwrap();

    let weight: f32 = current_weight.trim().parse().unwrap();

    let mars_weight: f32 = calculate_weight_on_mars(weight);

    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}
