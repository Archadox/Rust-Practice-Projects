use std::io;

fn main() {
	println!("Type \"Burger\" or \"burger\"");
	let mut input = String::new();
	io::stdin()
	    .read_line(&mut input)
	    .expect("idk lol");

//    println!("You typed {}", input);
    if input == "Burger\n" || input == "burger\n"{
        println!("Your word is burger! Congratulations!");
    } 
    else {
    	println!("Your word is not burger, stupid.")
    }
}
