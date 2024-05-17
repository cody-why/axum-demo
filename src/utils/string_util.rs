pub fn is_digit(str:String) -> bool{
	for ch in str.chars(){
		if ch.is_ascii_digit() == false {
			return false;
		}
	}
	return true;
}

pub fn right_find_under_line (ori_str:String) -> usize{
	let chars = ori_str.chars().rev();
	let count = chars.clone().count();

	for (i, item) in chars.enumerate() {
		if item == '_' {
			return count-i-1;
		}
	}
	return usize::MAX;
}
