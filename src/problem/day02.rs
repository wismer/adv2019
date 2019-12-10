use std::fs;

pub enum OpCode {
	Halt,
	// Opcode 1 adds together numbers read from two positions and
	// stores the result in a third position. The three integers
	// immediately after the opcode tell you these three positions
	//  - the first two indicate the positions from which you should read the input values,
	// and the third indicates the position at which the output should be stored.
	Add(isize, isize, isize), // OPCODE 1
	// OPCODE 2, same as 1 but multiplies
	Mult(isize, isize, isize),
	Save(isize, isize), // at location 1, save value of location 0
	Print(isize, isize), // at location 1, print out?? value at location 0
}

pub fn get_input(ignore_newlines: bool) -> Vec<isize> {
  let input: Vec<isize> = fs::read_to_string("./day05.txt")
	.unwrap()
	.as_mut_str()
	.split(",")
	.map(|c| c.parse::<isize>().unwrap())
	.collect();

	input
}

struct OPCReader {
	commands: Vec<isize>,
	current_index: usize,
}

impl OPCReader {
	pub fn new(commands: Vec<isize>) -> OPCReader {
		OPCReader {
			commands,
			current_index: 0,
		}
	}
}

// impl Iterator for OPCReader {
// 	type Item = OpCode;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		let mut index = self.current_index;
// 		let commands = self.commands;

// 		if index >= commands.len() {
// 			()
// 		}

// 		let opc = match commands[index] {
// 			1 => {
// 				self.current_index += 3;
// 				OpCode::Add(commands[index + 1], commands[index + 2], commands[index + 3])
// 			},
// 			2 => {
// 				self.current_index += 3;
// 				OpCode::Mult(commands[index + 1], commands[index + 2], commands[index + 3])
// 			},
// 			3 => {
// 				self.current_index += 2;
// 				OpCode::Save(commands[index + 1], commands[index + 2])
// 			},
// 			4 => {
// 				self.current_index += 2;
// 				OpCode::Print(commands[index + 1], commands[index + 2])
// 			},
// 			99 => OpCode::Halt,
// 			_ => panic!("invalid")
// 		};

// 		Some(opc)
// 	}
// }

pub fn day_two() {
	let contents = fs::read_to_string("./day2.txt");
	let input = match contents {
		Ok(file) => file,
		Err(_) => panic!("DOINK")
	};

	// let input = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
	let opcodes_str = &input.replace(",", "\n");
	let mut opcodes: Vec<isize> = vec![];
	for line in opcodes_str.lines() {
		let num = match line.parse::<isize>() {
			Ok(i) => i,
			Err(_) => panic!("NUM NOT PARSED")
		};

		opcodes.push(num);
	}

	let mut opcode_index = 0;
	// let mut a_index = 0;
	// let mut b_index = 0;
	// let mut c_index = 0;

	if let Some(num) = opcodes.get_mut(1) {
		*num = 39;
	};

	if let Some(num) = opcodes.get_mut(2) {
		*num = 51;
	};

	loop {
		if opcode_index >= opcodes.len() {
			break;
		}
		let opc = opcodes[opcode_index];
		let action = match opc {
			1 => OpCode::Add(opcodes[opcode_index + 1], opcodes[opcode_index + 2], opcodes[opcode_index + 3]),
			2 => OpCode::Mult(opcodes[opcode_index + 1], opcodes[opcode_index + 2], opcodes[opcode_index + 3]),
			3 => OpCode::Save(opcodes[opcode_index + 1], opcodes[opcode_index + 2]),
			4 => OpCode::Print(opcodes[opcode_index + 1], opcodes[opcode_index + 2]),
			99 => OpCode::Halt,
			_ => panic!("invalid")
		};

		let (new_value, position) = get_result(action, &opcodes);
		println!("SAJDghAKSJdhKSDJHD: {}", new_value);

		if new_value == 0 && position == 0 {
			println!("BREAK {}", opcode_index);
			break;
		}

		if let Some(num) = opcodes.get_mut(position as usize) {
			*num = new_value;
		}

		opcode_index += 4;
	}

	println!("FINAL {}", opcodes[0]);
}

fn get_result(operation: OpCode, values: &Vec<isize>) -> (isize, isize) {
	match operation {
		OpCode::Add(x, y, v) => {
			println!("ADD: x: {}, y: {}, v: {}", x, y, v);
			(values[x as usize] + values[y as usize], v)
		},
		OpCode::Mult(x, y, v) => {
			println!("MULT: x: {}, y: {}, v: {}", x, y, v);
			(values[x as usize] * values[y as usize], v)
		},
		_ => (0, 0)
	}
}