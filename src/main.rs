extern crate check_argument_cargo_io;
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
	println!("For a case insensetive result\n in Windows: before the program first type 'SET CASE=1' and then cargo run syntax\n for UNIX-base 'CASE=1 cargo run syntax'");
	
    let args_in_cmd: Vec<String> = env::args().collect();
	
	check_argument_cargo_io::valid_the_input::check_control(&args_in_cmd);
	
	let config = minigrep::Config::new_instance(&args_in_cmd);
    
	if let Err(e) = minigrep::run(config){
		println!("application error:{}", e);
		
		process::exit(0);
	};
}
