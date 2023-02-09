/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 22:12:22
 * @FilePath: /rustlings/exercises/strings/strings4.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

fn string_slice(arg: &str) {
	println!("{}", arg);
}
fn string(arg: String) {
	println!("{}", arg);
}

fn main() {
	// ???("blue");
	string_slice("blue");
	// ???("red".to_string());
	string("red".to_string());
	// ???(String::from("hi"));
	string(String::from("hi"));
	// ???("rust is fun!".to_owned());
	string("rust is fun!".to_owned());
	// ???("nice weather".into());
	string_slice("nice weather".into());
	string("nice weather".into());
	// ???(format!("Interpolation {}", "Station"));
	string(format!("Interpolation {}", "Station"));
	string_slice(&String::from("abc")[0..1]);
	// ???("  hello there ".trim());
	string_slice("hello there ".trim());
	// ???("Happy Monday!".to_string().replace("Mon", "Tues"));
	string("Happy Monday!".to_string().replace("Mon", "Tues"));
	// ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
	string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
