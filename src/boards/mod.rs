pub struct Board<DataType> {
	pub bus: Vec<Vec<[usize; 2]>>,
	pub components: Vec<String>,
	pub default_value: DataType
}

pub fn get_u8() -> Board<u8> {
	Board {
		bus: vec![vec![[0,1]]],
		components: vec![String::from("Test Component")],
		default_value: 20
	}
}