
pub trait Helper {
	fn number_of_characters_to_reveal(&self, v: &str) -> usize {
		match v.len() {
			0..=6   => 0,
			7..=10  => 1,
			11..=14 => 3,
			15..=18 => 5,
			19..=22 => 7,
			23..=26 => 9,
			_       => 12,
		}
	}
}

// blanket implementation for every type
impl<T> Helper for T {}
