// Exercise 3.3: Battery monitor — reference solution

fn main() {
    let robot = String::from("Ferris");
    let mut battery = 80;

    show_status(&robot, battery);

    charge(&mut battery, 15);
    show_status(&robot, battery);

    drain(&mut battery, 30);
    show_status(&robot, battery);

    // `battery` is an `i32`, which is `Copy`: keep a copy of the value
    // instead of a reference that would still be alive during `drain`.
    let before = battery;
    drain(&mut battery, 120);
    println!("Battery went from {before}% to {battery}%");

    show_status(&robot, battery);
}

fn show_status(name: &String, level: i32) {
    println!("{name}: battery at {level}%");
}

fn charge(level: &mut i32, amount: i32) {
    *level += amount;
}

fn drain(level: &mut i32, amount: i32) {
    if *level > amount {
        *level -= amount;
    } else {
        *level = 0;
    }
}
