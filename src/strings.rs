pub fn run(){

    // heap allocated
    let mut heapallocated = String::from("Hello");
    heapallocated.push('a');
    println!("Length, {}", heapallocated.len());

    //loop through strings by whitespace
    for word in heapallocated.split_whitespace(){
        println!("{}", word)
    }

    // Create string with capacity.
    let mut s = String::with_capacity(10);
    s.push('q');
    s.push('b');
    println!("{}", s);

    // Assertion Testing.
    assert_eq!(2, s.len());

    // Assertion Testing.
    assert_eq!(10, s.capacity())
}