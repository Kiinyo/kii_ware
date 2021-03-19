pub const CHECK: u128 = 170141183460469231731687303715884105728;

/// A collection of bit addresses stored as a single u128
pub trait State {
	
	// Accessing locations
	
	/// Return the state of an index
	fn get_index ( &mut self, index: usize ) -> bool;
	/// Set the state of an index.
	/// (returns the previous state)
	fn set_index ( &mut self, index: usize, on: bool ) -> bool;
	/// Inverts the state of an index.
	/// (returns the previous state)
	fn inv_index ( &mut self, index: usize ) -> bool;
	
	// Dealing with the states as a whole
	
	/// Returns a number of contiguous indexes represented as a u128
	fn get_state ( &mut self, index: usize, length: usize ) -> u128;	
	/// Sets a number of contiguous indexes represented as a u128.
	/// (returns previous states of the indexes represented as a u128)
	fn set_state ( &mut self, index: usize, length: usize, value: u128 ) -> u128;
	/// Flips every bit of a contiguous indexes
	/// (returns previous states of the indexes represented as a u128)
	fn inv_state ( &mut self, index: usize, length: usize ) -> u128;
	
}

pub struct Register {
	state: u128
}

impl State for Register {
	fn get_index ( &mut self, index: usize ) -> bool {
		self.state << index >= CHECK
	}
	fn set_index ( &mut self, index: usize, on: bool ) -> bool {
		if self.state << index >= CHECK {
			// Index is on
			if on {
				// Do nothing
			} else {
				self.state ^= CHECK >> index;
			}
			return true;
		} else {
			// Index is off
			if on {
				self.state |= CHECK >> index;
			} else {
				// Do nothing
			}
			return false;
		}
	}
	fn inv_index ( &mut self, index: usize ) -> bool {
		let boolean = !self.get_index(index);
		self.set_index( index, boolean )
	}
	
	fn get_state ( &mut self, index: usize, length: usize ) -> u128 {
		let mut previous: u128 = 0;
		for x in 0..length {
			if self.state << 127 - x - index >= CHECK {
				// The original index is on				
				previous += 1u128 << x;
			} else {
				// The original index is off
			}
		}
		return previous;
	}
	fn set_state ( &mut self, index: usize, length: usize, value: u128 ) -> u128 {
		let mut previous: u128 = 0;
		for x in 0..length {
			if self.state << x + index >= CHECK {
				// The original index is on				
				previous += 1u128 << x;
				if value << 127 - x - index >= CHECK {
					// The new index is on
					// Do nothing
				} else {
					// The new value is off
					self.state ^= 1u128 << 127 - x - index;
				}
			} else {
				// The original index is off
				if value << 127 - x >= CHECK {
					// The new index is on
					self.state |= 1u128 << 127 - x - index;
				} else {
					// The new value is off
					// do nothing
				}
			}
		}
		return previous;
	}
	fn inv_state ( &mut self, index: usize, length: usize ) -> u128 {
		let mut previous: u128 = 0;
		for x in 0..length {
			if self.state << 127 - x - index >= CHECK {
				// The original index is on				
				previous += 1u128 << x;
				self.state ^= 1u128 << 127 - x - index;
			} else {
				// The original index is off
				self.state |= 1u128 << 127 - x - index;
			}
		}
		return previous;
	}		
}

fn main () {
	println! ("Heyo!");		
}