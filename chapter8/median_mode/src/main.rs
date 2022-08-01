use std::collections::HashMap;

const VALUES: [i32; 10] = [1,2,3,4,55,9,1,2,0,1];
fn main() {
    println!("Median: {}", median(&VALUES));
    println!("Mode:   {}", mode(&VALUES));
}

fn median(values: &[i32]) -> f32 {
    let mut sum = 0;
    for value in values {
        sum += value;
    }
    return sum as f32 / values.len() as f32;
}

fn mode(values: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for value in values.iter() {
        let count = map.entry(value).or_insert(1);
        *count += 1;
    }
    let mut max_count = 0;
    let mut max_value = 0;
    for ( value, count ) in map {
        if count > max_count {
            max_count = count;
            max_value = *value;
        }
    }
    return max_value;
}
