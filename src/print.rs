pub fn run() {
    //printing to console
    println!("Hello from print.rs");
    //Basic Formatting
    println!("Number: {}", 1);

    println!("{} says hello to {}", "Batman", "Superman");

    //Positional argument
    println!(
        "{0} and {1} fought with each other and {0} won",
        "Batman", "Superman"
    );

    //Namd Argument
    println!(
        "My name is {name} and I am {dash} man alive",
        name = "Barry Allen",
        dash = "fastest"
    );

    //Placeholder trait
    println!("Binary : {:b} Hexa : {:x} Octa : {:o}", 20, 7892, 88);

    //Placeholde for debug traits
    println!("{:?}", (12, true, 89));

    //Basic math
    println!("10 * 10 = {}", 10 * 10);
}
