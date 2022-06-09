pub fn run(){
    let age = 22;
    let check_id = false;
    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("You are not allowed to drink....")
    }
    else{
        println!("I will need to see your id")
    }

    //shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}", is_of_age);
}