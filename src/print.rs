pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic format
    println!("{} is from {}", "Trinh Hung", "Hanoi");

    // Postional Argumentation
    println!("{0} is from {2} likes to {1}", "Trinh Hung", "playing chess", "Hanoi");

    // Named Arguments
    println!("{name} is from {loc} likes to {acti}", 
        name = "Trinh Hung", acti = "chess", loc = "Hanoi");

    // Placeholder traits
    println!("Binary: {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debuf trait
    println!("{:?}", (12, true, "hello"));
}