#[derive(Debug, Clone)]
pub struct Combinator {
	pub digits: Vec<u32>,
	pub radix: u32,
}

impl Combinator {
	pub fn new(digit_count: usize, radix: u32) -> Combinator {
		return Combinator {
			digits: vec![0; digit_count],
			radix: radix,
		};
	}
	pub fn len(&self) -> usize {
		return self.digits.len();
	}
	pub fn get(&self, index: usize) -> usize {
		return self.digits[index] as usize;
	}
	pub fn shift(&mut self, delta: isize) -> isize {
		let isz_radix: isize = self.radix as isize;
		let mut carry = delta;
		for i in 0..self.digits.len() {
			// Account for the existing digit value
			let v = carry + self.digits[i] as isize;
			// If value causes overflow...
			if v < 0 || v >= isz_radix {
				// Shave off the radix modulus from the value
				let digit = ((v % isz_radix) + isz_radix) % isz_radix;
				// Assign it as the current digit
				self.digits[i] = digit as u32;
				// Truncate the digit from the existing value
				carry = (v - digit) / isz_radix;
			} else {
				self.digits[i] = v as u32;
				break;
			}
		}
		return carry;
	}
}
