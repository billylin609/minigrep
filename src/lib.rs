use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::env;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	/*hold the logic for parsing a vec to val*/
	pub fn new_instance<'a> (args: &'a Vec<String>) -> Vec<Config> {
		let mut the_whole_set_pair = Vec::new(); 
		let v = env::var("USER").expect("$USER is not set");
		let case_sensitive = env::var("CASE").is_err();
		//println!("{}", case_sensitive);
		/* the later version is assume only have one string and one file correspond*//*----adoption required-----*/
		let mut i = 1;
		while i + 1 <= args.len() {
		let query = args[i].clone();
		let filename = args[i+1].clone();
		the_whole_set_pair.push(Config{query, filename, case_sensitive});
		i += 2;
		}
		//these two trade off the peformance with a straightforward no lifetime spcifier program
	
		the_whole_set_pair//2 pair of query and filename ret
	}
}

pub fn run (configs: Vec<Config>) -> Result<(), Box<Error>> {//()is the unit type contain no val
	for config in configs {
	println!("\n");
	fs::write("helloworld.txt", "poem is i\n")?;//write words into document

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    let mut f = File::open(config.filename)?;//get the file holder and check for file existence

    let mut contents = String::new();//var to hold file 
    f.read_to_string(&mut contents)?;//read the content //with Result<Ok(), Err>
		
	//println!("with text:\n{}", contents);
	let mut result = Vec::new();
	
	if config.case_sensitive {
		result = search(&config.query, &contents);
	} else {
		result = search_case_insensitive(&config.query, &contents);
	};

	for line in result {//a word may apppear more than one{we might see more than one item in vec}
		println!("{}", line);
	}
	println!("\n");
	}
	
	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {//return a slice of the code for content doc
	let mut result = Vec::new();
	
	for line in contents.lines() {
		if line.contains(query) {
			result.push(line);
		}
	}
	
	result
}

pub fn search_case_insensitive<'a>(query: &str, content:&'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	
	let mut result = Vec::new();
	
	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
			result.push(line);
		}
	}
	
	result
}	

#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
		assert_eq!(vec!["safe, fast, productive."],
		search(query, contents)
		);
	}
}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(vec!["Rust:", "Trust me."],
		search_case_insensitive(query, contents)
		);
}