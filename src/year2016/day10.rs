use std::cell::RefCell;

#[allow(dead_code)]
pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(dead_code)]
struct Robot {
	hands:Vec<i32>,
	target_hi:Option<i32>,
	target_lo:Option<i32>,
	robots:RefCell<Vec<Robot>>,
}
/*
impl Robot {
	fn create(robots:RefCell<Vec<Robot>>) -> Self {
		let robot = Robot {
			hands:Vec::new(),
			target_hi:None,
			target_lo:None,
			robots:robots,
		};
		robot.robots.borrow_mut().push(robot);
		robot
	}

	fn take(&mut self, num:i32) -> bool {
		if self.hands.len()>=2 {
			panic!("robot already full!");
		}
		if self.hands.is_empty() {
			self.hands.push(num);
		} else {
			if self.hands[0]<num {
				self.hands.push(num);
			} else {
				self.hands.insert(0,num);
			}
		}
		self.process()
	}

	fn set_hi(&mut self, num:i32) -> bool {
		match self.target_hi {
			Some(_) => panic!("Hi already set"),
			None    => self.target_hi = Some(num),
		}
		self.process()
	}

	fn set_lo(&mut self, num:i32) -> bool {
		match self.target_lo {
			Some(_) => panic!("Lo already set"),
			None    => self.target_lo = Some(num),
		}
		self.process()
	}

	fn process(&mut self) -> bool {
		if   self.hands.len()==2 
		  && self.target_hi.is_some()
		  && self.target_lo.is_some() {
			  if self.hands[0]==17
			    && self.hands[1]==61 {
					true
			  } else {
				  self.robots.borrow()[self.target_lo.unwrap() as usize].take(self.hands[0]);
				  self.robots.borrow()[self.target_hi.unwrap() as usize].take(self.hands[1]);
				  self.hands.clear();
				  self.target_lo = None;
				  self.target_hi = None;
				  false
			  }
	    } else { 
			false
		}
	}
}
*/