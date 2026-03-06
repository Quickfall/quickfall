#[macro_export]
macro_rules! info {
	($exp:expr) => {
		printf!("INFO: " + exp);	
	};
}

#[macro_export]
macro_rules! debug {
	($exp:expr) => {
		printf!("DEBUG: " + exp);	
	};
}
