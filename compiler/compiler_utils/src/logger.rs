#[macro_export]
macro_rules! info {
	($exp:expr) => {
		println!("INFO: " + exp);	
	};
}

#[macro_export]
macro_rules! debug {
	($exp:expr) => {
		println!("DEBUG: " + exp);	
	};
}
