/*---This struct can be useful but in this case use result can handle the value and exit as user friendly way---*/
/*pub struct ConfigForRet {
	pub response: bool,
	pub err_loc: i32,
}*/

use std::process;//unused import

/*
/*cooperate with the terminal input*//* move out for program clarity*/
pub fn valid_io(args_for: & Vec<String>) {
	let result = check_control(&args_for);
    if result.response == false {
        if result.err_loc == 1 {
            panic!("number of item not match");
        } else {
            println!("the input {} is not a file dire", result.err_loc - 2);
            panic!("the item of collect is not desired");
        }
    } else {
        println!("success");
    }
}*/

//summary all in one
pub fn check_control(all_args: &Vec<String>) {//in this case return arry [type; number of item]
	
    let fn_sc_result = check_leng(all_args).unwrap_or_else(|err|{//check for odd number of item
		eprintln!("problem with parsing the argument: {}", err);//display output
		process::exit(0);//sucess exit
	});
	drop(fn_sc_result);
    //println!("watch here{} {}",return_val.0, return_val.1);
    let fn_sc_result = check_file_dir(all_args).unwrap_or_else(|err|{//check for odd number of item
		eprintln!("problem with parsing the argument: {}", err);
		process::exit(0);
	});
	drop(fn_sc_result);
		//println!("sucess!");
	//result
}

/*fn call_exit(err_output: &'static str) {
	println!("problem with parsing the argument: {}", err_output);
	process::exit(0);
}*///private API when not used

fn check_leng (val_in_list: &Vec<String>) -> Result<(), &'static str> {
    let to_len = val_in_list.len();//fetch item in list
	//println!("{}", to_len);
	if to_len == 1 {//containing the orgininal file path
		return Err("This input not contain any of the val.\nSuggested syntax: cargo run String_needed target_file.txt.");
	}
	
    if to_len % 2 == 0 {
		return Err("number of input following cargo run are not in pairs.");
        //false//even number of item(not match include orginal file path)
    } else {
        return Ok(());//odd number of item)[in pairs]
    }
}

fn check_file_dir (val_in_list: &Vec<String>) -> Result<(), String> {
    let to_len = val_in_list.len();//fetch item in list
    let mut count_down = (to_len - 1) / 2;//items in dir file form
    let mut list_i = 2;// the third item should be the first dir
    let mut rep_item = 1;//rep the number of item it is accessing
    //println!("count_down:{}", count_down);
    while count_down > 0 {
        let mut contain_wanted = 0;//check contain or not a "."
        //println!("{}", val_in_list[list_i]);
        for chars in val_in_list[list_i].chars() {
            //println!("{}", chars);
            if chars == '.'
            {
                contain_wanted = 1;
                break;
            }
        }
        if contain_wanted != 1 {
            let return_str = format!("The directory with number {} is not a valid a file type", rep_item);
			return Err(return_str);
        }
        rep_item += 1; 
        list_i = list_i + 2;
        count_down -= 1;
    }
    Ok(())
}


#[cfg(test)]
use super::*;

#[test]
fn check_panic() {
    let a_val = vec!("hi".to_string(), "why".to_string(), "you".to_string());
	assert_eq!(check_control(&a_val).response, false);
	assert_eq!(check_control(&a_val).err_loc, 1+2);
}

#[test]
fn check_not_panic() {
    let a_val = vec!("hi".to_string(), "why".to_string(), "you.txt".to_string());
	assert_eq!(check_control(&a_val).response, true);
	assert_eq!(check_control(&a_val).err_loc, 0);
    //assert_eq!(check_control(&a_val), {true, 0});
}

#[test]
fn more_string_check_panic() {
    let a_val = vec!("hi".to_string(), "why".to_string(), "you".to_string(), "why".to_string(), "why".to_string());
    assert_eq!(check_control(&a_val).response, false);
	assert_eq!(check_control(&a_val).err_loc, 1+2);
	//assert_eq!(check_control(&a_val), {false, 1+2});
}

#[test]
fn more_string_check_for_panic() {
    let a_val = vec!("hi".to_string(), "why".to_string(), "you.txt".to_string(), "why".to_string(), "why".to_string());
	assert_eq!(check_control(&a_val).response, false);
	assert_eq!(check_control(&a_val).err_loc, 2+2);
    //assert_eq!(check_control(&a_val), {false, 2+2});
}

#[test]
fn check_not_more_str_panic() {
    let a_val = vec!("hi".to_string(), "why".to_string(), "you.txt".to_string(), "and".to_string(), "and.txt".to_string());
    assert_eq!(check_control(&a_val).response, true);
	assert_eq!(check_control(&a_val).err_loc, 0);
	//assert_eq!(check_control(&a_val), {true, 0});
}