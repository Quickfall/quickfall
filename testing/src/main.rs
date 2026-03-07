//! A rudimentary testing suite for Quickfall 

use std::fs;

use crate::utils::run_test;

pub mod utils;

pub fn run_tests_in(path: String) -> bool {
	let paths = fs::read_dir(path).unwrap();

	for path in paths {
		let p = path.unwrap();
		let real = p.path().as_os_str().to_str().unwrap().to_string();

		if p.file_type().unwrap().is_dir() {
			if !run_tests_in(real) {
				return false;
			}
		} else {
			if !run_test(real) {
				return false;
			}
		}
	}

	return true;
}

pub fn main() {
	run_tests_in("./tests".to_string());
}