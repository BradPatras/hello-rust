// Single line comments woo

/*
Comment blocks woooooo...
oooooo
 */

fn main() {
    println!("Hey Rust?");

    println!("Wow {} arguments", "Rust has");

    println!("Woah {lang}, {adjective} moves", 
             lang="Rust",
             adjective="nice");
    
    println!("hmm formatting on the fly {} == {:b}", 10, 2);

    // Prints 1 padded with 5 white spaces = "     1"
    println!("{number:>width$}", number=1, width=6);
    
    // Prints 1 padded with 5 zeros = "000001"
    println!("{number:>0width$}", number=1, width=6);

    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
}


