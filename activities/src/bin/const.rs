const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}

fn main() {
    let speed_one = clamp_speed(40);
    let speed_two = clamp_speed(9400);

    println!("s1={:?}", speed_one);
    println!("s2={:?}", speed_two);
}
