//use time;
use std::path;
use std::fs;
use std::env;
use std::env::var;


fn main() {
    check_config_directory();
}

fn check_config_directory() { //this function checks to see if the config directory exists

    let existence = std::path::Path::new("~/.alarmy_rust")
        .exists();
    if existence == true {

    }
    else if existence == false {
        config_creation();

    }

}

fn config_creation() {
    let e = std::env::var!("USER");
    println!("creating config");
    fs::create_dir("$HOME/.alarmy_rust")
    .expect("Something's broken idfk");
    fs::create_dir("$HOME/.alarmy_rust/alarms")
    .expect("Something's broken idfk");
}
