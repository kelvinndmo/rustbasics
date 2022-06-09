pub fn run(){
    //print to console.
    println!("Hello from print.rs file!");

    // Basic formating
    println!("{} days", 31);
    println!("{} is from {}","Kelvin", "Keroka");

    //positional arguments
    println!("{0} is from {1} and likes to {2}", "Kelvin","Keroka", "code");

    //named arguements.
    println!("{name} likes to play {activity}", name="John", activity="Tennis");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x}, Octal:{:o}",10,10,10);

    //placeholder for debug traits
    //tuple
    println!("{:?}", (12, true, "hello"));

    //Basic match
    println!("10 + 10 = {}", 10 + 10)
}