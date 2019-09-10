use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Enter the roll you are looking for:");
    println!("Example: 2d6");

    //Initializing string to store user input
    let mut desired_roll = String::new();

    //Retrieving user input from CLI
    io::stdin().read_line(&mut desired_roll)
        .expect("Failed to read input");

    //Splitting user input into the two relevant parts - the number and type of the dice
    let desired_roll: Vec<&str> = desired_roll.split("d").collect();
    //Parsing each part into a number and storing in separate variables
    let iterator= desired_roll[0].trim().parse::<i32>().unwrap();
    let die = desired_roll[1].trim().parse::<i32>().unwrap();

    //Assert statement as sanity check.
    //assert_eq!(desired_roll, [2,6]);

    //Displaying min/max for the desired roll
    println!("Max possible value is {}, and min possible value is {}", iterator*die, iterator);

    //Initializing variables necessary for loop
    let mut iteration = 0;
    let mut sum = 0;
    let mut rolls: Vec<i32> = Vec::new();

    //Loop to 'roll' each die
    loop{
        //Incrementing the counter
        iteration = iteration + 1;

        //Rolling the die
        let roll = rand::thread_rng()
            .gen_range(1,die+1);

        //Displaying each roll in case the rolls individually matter
        println!("Roll {}: {}", iteration, roll);

        //Incrementing sum by the roll
        sum = sum + roll;

        //Pushing the roll to the rolls Vec as courtesy to myself if I decide to upgrade
        //functionality
        rolls.push(roll);

        //Break if loop counter reaches or surpasses user input
        match iteration.cmp(&iterator) {
            Ordering::Greater => break,
            Ordering::Equal => break,
            Ordering::Less => continue
        }

    }
    //Displaying final result
    println!("The sum was {}", sum);
}
