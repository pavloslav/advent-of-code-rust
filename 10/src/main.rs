struct Robot {
	hi:i32,
	lo:i32,
	target_hi:usize,
	target_lo:usize,
}

impl Robot {
	fn new() -> Self {
		Robot {
			hi:-1,
			lo:-1,
			target_hi:-1,
			target_lo:-1,
		}
	}
	
	fn give(&self, num:i32){
		if self.hi>-1 && self.lo>-1 {
			panic!("robot already full!");
		}
		if num>self.hi {
			self.lo = self.hi;
			self.hi = num;
		} else {
			

		}
	}
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini(env!("CARGO_PKG_NAME")).unwrap();
     println!("Hello, world!");
}
