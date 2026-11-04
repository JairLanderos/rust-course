// Exercise 3.3: Battery monitor
//
// The tasks are in README.md. Check your work with:
//     cargo test -p s03-l03-references-borrowing

fn main() {
    let robot = String::from("Ferris");
    let mut battery = 80;

    show_status(robot, battery);

    charge(battery, 15);
    show_status(robot, battery);

    drain(&mut battery, 30);
    show_status(robot, battery);

    let before = &battery;
    drain(&mut battery, 120);
    println!("Battery went from {before}% to {battery}%");

    show_status(robot, battery);
}

// Task 1: printing a status shouldn't take the robot's name away from `main`.
fn show_status(name: String, level: i32) {
    println!("{name}: battery at {level}%");
}

// Task 2: charging must change the battery level in `main`.
fn charge(level: i32, amount: i32) {
    level += amount;
}

// Task 4: take `amount` away from the level, but never go below 0.
fn drain(level: &mut i32, amount: i32) {}
