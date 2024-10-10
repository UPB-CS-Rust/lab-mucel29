fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut min: i32 = i32::MAX;
    let mut max: i32 = i32::MIN;
    for n in input {
        if n > max {
            max = n
        } else if n < min {
            min = n
        }
    }

    println!("{} is largest and {} is smallest", max, min);
}
