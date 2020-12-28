pub fn update(i: u8, increase: bool) -> u8 {
	if increase {
		return i + 1;
	} else {
		return i - 1;
	}
}