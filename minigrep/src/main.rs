use std::{env, process, };
use colored::*;

use minigrep::Config;
use minigrep::run;

fn main() {

    let args: Vec<String> = env::args().collect();

   let config = Config::new(&args).unwrap_or_else( |err| {
        eprintln!("{} {}", "Problem parsing arguments mate:".red(), err.red());
        process::exit(1);
   });

   println!("Searching for {} in {}",config.query,config.filename);
    
   if let Err(e) = run(config){
    eprintln!("App error: {}",e);
    process::exit(1)
   } 
}


