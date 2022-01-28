use ansi_term::*;
use chrono::prelude::*;
use std::env;
use env_logger::{Builder, Target};

fn strftime() -> String { 
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}


pub fn init() -> () {
    let mut builder = Builder::from_default_env();

    builder.target(Target::Stdout);

    println!("{}",strftime());

    ()

}



#[cfg(test)]
mod tests {
    use super::*;    
    #[test]
    fn it_works() {
        init();

    }


}
