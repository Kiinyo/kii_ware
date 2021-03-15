use std::sync::{ Arc, Mutex };

pub struct Component<DataType> {
	pub pin_number: usize,
	pub reg_number: usize,
	
	pub pins: Vec<Arc<Mutex<DataType>>>,
	pub registers: Vec<DataType>,
	
	pub default_value: DataType,
	
	pub logic: fn (
		&mut Vec<Arc<Mutex<DataType>>>,
		&mut Vec<DataType>
		)
}

pub mod test_component;
// USER COMPONENTS: Add the filename after this line $

pub fn create_u8 ( component_name: String ) -> Component<u8> {
	match component_name.as_str() {
		"Test Component" => {
			Component {
				pin_number: test_component::PIN_NUMBER,
				reg_number: test_component::REG_NUMBER,
				
				pins: Vec::new(),
				registers: Vec::new(),
				
				default_value: test_component::DEFAULT_VALUE,
				logic: test_component::logic
			}
		},
		// USER COMPONENTS: Add components after this line $
		
		_ => { panic! ("Invalid Name: \"{}\"", component_name) }
	}
}
				