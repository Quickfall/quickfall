pub fn get_signed_highbound(bits: usize) -> i128 {
	return 2_i128.pow(bits as u32 - 1) - 1;
}

pub fn get_signed_lowbound(bits: usize) -> i128 {
	return 0 - 2_i128.pow(bits as u32 - 1) - 1;
}

pub fn get_unsigned_highbound(bits: usize) -> i128 {
	return 2_i128.pow(bits as u32) - 1;
}

pub fn can_num_fit_inbits_signed(bits: usize, num: i128) -> bool {
	return num >= get_signed_lowbound(bits) && num <= get_signed_highbound(bits);
}

pub fn can_num_fit_inbits_unsigned(bits: usize, num: i128) -> bool {
	return num >= 0 && num <= get_unsigned_highbound(bits);
}
