use std::io;
use std::str::FromStr;

fn parse(){

}

fn main() {
    println!("Enter the roll you are looking for:");
    println!("Example: 2d6");

    let mut desired_roll = String::new();

    io::stdin().read_line(&mut desired_roll)
        .expect("Failed to read input");

    let mut desired_roll: Vec<&str> = desired_roll.split("d").collect();
    desired_roll[0] = desired_roll[0].trim().parse().unwrap();
    desired_roll[1] = desired_roll[1].trim().parse().unwrap();
    assert_eq!(desired_roll, [2,6]);
    //let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();

    println!("{} {}", desired_roll[0], desired_roll[1]);
}
