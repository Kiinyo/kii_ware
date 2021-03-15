pub const DEFAULT_VALUE: u8 = 0;
pub const PIN_NUMBER: usize = 2;
pub const REG_NUMBER: usize = 1;

use std::sync::{Arc, Mutex};

pub fn logic ( pin: &mut Vec<Arc<Mutex<u8>>>, register: &mut Vec<u8> ) {
	
	std::thread::sleep(std::time::Duration::from_secs(1));
	
	let input_pin = pin[0].lock().unwrap();	
	let input_value = *input_pin;
	drop(input_pin);	
	
	let mut output_pin = pin[1].lock().unwrap();
	
	if register[0] == 0 {
		register[0] = 100;
		*output_pin = 10;
		drop(output_pin);
		println! ( "Test Component: Starting up!" );
		println! ( "Test Component: Input pin = {}", input_value);
	} else if *output_pin > 0 {
		let output_value = *output_pin;
		*output_pin -= 1;
		drop(output_pin);
		println! ( 
			"Test Component: Stepping down from {} to {}",
			output_value,
			output_value - 1
		);
	} else {
		panic! ( "Test Component: Shutting down..." );
	}
}
		