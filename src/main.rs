use std::sync::{ Arc, Mutex };

pub struct Pins {
	pre: u128,             // Previous pin value
	pub val: u128,             // Current pin value
	bus: Arc<Mutex<u128>>, // Bus pins are connected to
	pub con: [usize;128],      // List of all the connections
	len: usize             // How many pins are used
}

impl Pins {
	// Get the on/off of the pin
	fn get (&self, pin: usize) -> bool {
		170141183460469231731687303715884105728 <= self.val << 127 - pin
	}
	// Set the on/off of the pin
	fn set (&mut self, pin: usize, on: bool) {
		if 170141183460469231731687303715884105728 <= self.val << 127 - pin {
			// The pin is on
			if on {
				// Do nothing, it's on
			} else {
				// Flip it
				self.val ^= 1_u128 << pin;
			}
		} else {
			// The pin is off
			if on {
				// Flip it
				self.val |= 1_u128 << pin;
			} else {
				// Do nothing
			}
		}
	}
	fn refresh (&mut self) {
		// Get changes
		let cha = self.val ^ self.pre;
		let mut b = self.bus.lock().unwrap();
		// Refresh pins
		for x in 0..self.len {
			// We're writing a change
			if 170141183460469231731687303715884105728 <= cha << 127 - x {
				// A change has occurred on this pin
				if self.get(x) {
					// The change is On
					if *b << 127 - self.con[x] >= 170141183460469231731687303715884105728 {
						// Do nothing, the bus is already on
					} else {
						*b |= 1_u128 << self.con[x]
					}
				} else {
					// The change is Off
					if *b << 127 - self.con[x] >= 170141183460469231731687303715884105728 {
						*b ^= 1_u128 << self.con[x]
					} else {
						// Do nothing, the bus is already off
					}					
				}
			}
			// We're updating our pins
			else {
				if self.get(x) {
					// Our pin is on
					if *b << 127 - self.con[x] >= 170141183460469231731687303715884105728 {
						// Do nothing, the bus is on too
					} else {
						self.val ^= 1_u128 << x;
					}
				} else {
					// Our pin is off
					if *b << 127 - self.con[x] >= 170141183460469231731687303715884105728 {
						self.val |= 1_u128 << x;
					} else {
						// Do nothing, the bus is off too
					}					
				}
			}			
		}
		drop(b);
		self.pre = self.val;
	}
}

fn main () {
	let mut bus = Arc::new(Mutex::new(5)); // ...101
	let mut pins = Pins { // ...000
		pre: 0,
		val: 0,
		bus: Arc::clone(&bus),
		con: [0; 128],
		len: 3
	};
	pins.con[0] = 0;
	pins.con[1] = 1;
	pins.con[2] = 2;
	
	println!("Pins: {}", pins.val);
	
	pins.refresh();
	
	println!("Pins: {}", pins.val); // ...101
	
	pins.set(1, true); // ...111
	let mut b = bus.lock().unwrap();
	*b = 0;
	drop(b); // ...000
	pins.refresh();
	
	let mut b = *bus.lock().unwrap();
	
	println!("Bus: {}", b); // ...010
	
		
}