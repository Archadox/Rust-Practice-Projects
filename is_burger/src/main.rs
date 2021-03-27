use std::io; //le import

fn main() { //the main function of the code
	println!("Type \"Burger\" or \"burger\", then Enter"); //tells the user what to type
	let mut input = String::new(); //creates the varable to save user input to
	io::stdin() 
	    .read_line(&mut input) //read the input of the user and saves it to the "input" variable
	    .expect("idk lol"); //something to prevent error lmao

//    println!("You typed {}", input); //commented out rn, but used for testing mainly
    if input == "Burger\n" || input == "burger\n"{ //comparing the user's input to the word "burger", capitalized and not
        println!("Your word is burger! Congratulations!"); //response when the user correctly types the word "burger"
    } 
    else {
    	println!("Your word is not burger, stupid.") //when the user is idiotic and types something other than "burger"
    }
}
