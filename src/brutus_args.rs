#[derive(Debug, Clone)]
pub struct BrutusArgs {
	// Execution path that the binary was run from
	pub path: String,
	// Optional starting prefix - if the prefix is 3 characters and the length is
	// 8 characters, the combinator will only rotate through the last 5 characters
	pub prefix: String,
	// The allowed set of characters to iterate over.
	pub charset: String,
	// Optional target string to search for
	pub target_str: String,
	// The target length of the generated output strings.
	pub length: usize,
	// The number of generated strings of length "length" the barrage should spit out.
	pub count: usize,
	// How many parallel barrages we should run.
	pub swarm: usize,
}

impl BrutusArgs {
	fn parse_positive_int(name: &str, value: String) -> usize {
		match value.parse::<usize>() {
			Ok(i) => return if i > 0 { i } else { 1 },
			Err(_) => panic!(
				"--{} parameter must be a positive integer - received '{}'",
				name, value
			),
		}
	}
	fn expand_charset_pattern(pattern: String) -> String {
		let mut result = String::new();
		if pattern.contains("a") {
			result.push_str("abcdefghijklmnopqrstuvwxyz");
		}
		if pattern.contains("A") {
			result.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
		}
		if pattern.contains("1") {
			result.push_str("0123456789");
		}
		if pattern.contains("!") {
			result.push_str("!@#$%^&*()_+-={};\"' <>,./\\?|`~");
		}
		return result;
	}
	fn put_charset(&mut self, charset: String) {
		self.charset = charset;
		self.swarm = self.charset.len();
	}
	fn put(&mut self, key: String, value: String) {
		if key.eq("--charset") {
			self.put_charset(value);
		} else if key.eq("--charset-pattern") {
			self.put_charset(BrutusArgs::expand_charset_pattern(value));
		} else if key.eq("--prefix") {
			self.prefix = value;
		} else if key.eq("--target") {
			self.target_str = value;
			self.length = self.target_str.len();
		} else if key.eq("--length") {
			self.length = BrutusArgs::parse_positive_int(key.as_str(), value);
		} else if key.eq("--count") {
			self.count = BrutusArgs::parse_positive_int(key.as_str(), value);
		} else if key.eq("--swarm") {
			self.swarm = BrutusArgs::parse_positive_int(key.as_str(), value);
		}
	}
	pub fn char_at(&self, index: usize) -> Option<char> {
		return self.charset.chars().nth(index);
	}
	pub fn get_template_str(&self) -> String {
		return String::with_capacity(self.length + self.prefix.len());
	}
	pub fn has_target(&self) -> bool {
		return !self.target_str.is_empty();
	}
	pub fn max_iterations(&self) -> usize {
		return if self.has_target() {
			u32::MAX as usize
		} else {
			self.count
		};
	}
	pub fn new(args: Vec<String>) -> BrutusArgs {
		let mut result = BrutusArgs {
			path: String::from(""),
			charset: String::from(""),
			prefix: String::from(""),
			target_str: String::from(""),
			count: 1,
			length: 1,
			swarm: 1,
		};
		result.path = args.get(0).unwrap().to_string();
		for chunk in args[1..].chunks(2) {
			if chunk.len() != 2 {
				continue;
			}
			let key = chunk.get(0).unwrap().to_string();
			let value = chunk.get(1).unwrap().to_string();
			result.put(key, value);
		}
		return result;
	}
}
