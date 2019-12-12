use std::fs;
use permutohedron::heap_recursive;
#[derive(Debug, Clone, Copy)]
pub enum ParameterMode {
	Immediate,
	Position,
	None
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	Halt,
	// Opcode 1 adds together numbers read from two positions and
	// stores the result in a third position. The three integers
	// immediately after the opcode tell you these three positions
	//  - the first two indicate the positions from which you should read the input values,
	// and the third indicates the position at which the output should be stored.
	Add, // OPCODE 1
	// OPCODE 2, same as 1 but multiplies
	Mult,
	Save, // at location 1, save value of location 0
	Print, // at location 1, print out?? value at location 0
	JumpIfTrue,
	JumpIfFalse,
	LessThan,
	Equals,
}

#[derive(Debug)]
pub struct Operation {
	pub command: OpCode,
	pub parameter_modes: [ParameterMode; 2],
}

impl Operation {
	pub fn jump(&self, index: usize, jump_if_true: bool, commands: &Vec<isize>) -> usize {
		let first_param = self.get_value(index + 1, self.parameter_modes[1], commands);
		if jump_if_true && first_param == 0 || !jump_if_true && first_param != 0 {
			return index + 3
		}

		let v = self.get_value(index + 2, self.parameter_modes[0], commands) as usize;
		v
	}

	pub fn less_than_or_equal_to(&self, opcode: OpCode, index: usize, commands: &mut Vec<isize>) -> usize {
		let p1 = self.get_value(index + 1, self.parameter_modes[1], commands);
		let p2 = self.get_value(index + 2, self.parameter_modes[0], commands);

		let value = match opcode {
			OpCode::LessThan => {
				if p1 < p2 {
					1
				} else {
					0
				}
			},
			OpCode::Equals => {
				if p1 == p2 {
					1
				} else {
					0
				}
			},
			_ => unreachable!("asdhjka")
		};

		// println!("<= VALUE {} with {} and {} write to: {}", value, p1, p2, p3);
		let i = commands[index + 3];
		if let Some(v) = commands.get_mut(i as usize) {
			*v = value;
		};

		println!("p1 {} p2 {} p3 {}", p1, p2, i);

		index + 4
	}

	fn get_value(&self, index: usize, mode: ParameterMode, commands: &Vec<isize>) -> isize {
		match mode {
			ParameterMode::Immediate => commands[index],
			_ => {
				let i = commands[index] as usize;
				commands[i]
			}
		}
	}

	pub fn mult(&self, index: usize, commands: &mut Vec<isize>) -> usize {
		let modes: [ParameterMode; 2] = self.parameter_modes;
		let v1: isize = self.get_value(index + 1, self.parameter_modes[1], commands);
		// let v1: isize = match modes[1] {
		// 	ParameterMode::Immediate => commands[index + 1],
		// 	_ => {
		// 		let i = commands[index + 1] as usize;
		// 		commands[i]
		// 	}
		// };
		// let v2: isize = match modes[0] {
		// 	ParameterMode::Immediate => commands[index + 2],
		// 	_ => {
		// 		let i = commands[index + 2] as usize;
		// 		commands[i]
		// 	}
		// };
		let v2: isize = self.get_value(index + 2, modes[0], commands);
		let result_index = commands[index + 3] as usize;
		if let Some(v) = commands.get_mut(result_index) {
			*v = v1 * v2;
		};

		index + 4
	}

	pub fn add(&self, index: usize, commands: &mut Vec<isize>) -> usize {
		let modes: [ParameterMode; 2] = self.parameter_modes;
		let v1: isize = self.get_value(index + 1, self.parameter_modes[1], commands);
		let v2: isize = self.get_value(index + 2, modes[0], commands);
		// let v1: isize = match modes[1] {
		// 	ParameterMode::Immediate => commands[index + 1],
		// 	_ => {
		// 		let i = commands[index + 1] as usize;
		// 		commands[i]
		// 	}
		// };
		// let v2: isize = match modes[0] {
		// 	ParameterMode::Immediate => commands[index + 2],
		// 	_ => {
		// 		let i = commands[index + 2] as usize;
		// 		commands[i]
		// 	}
		// };
		let result_index = commands[index + 3] as usize;
		if let Some(v) = commands.get_mut(result_index) {
			*v = v1 + v2;
		};

		index + 4
	}

	pub fn save(&self, index: usize, commands: &mut Vec<isize>, value: isize) -> usize {
		let i = commands[index + 1];

		if let Some(v) = commands.get_mut(i as usize) {
			*v = value;
		};

		index + 2
	}

	pub fn print(&self, index: usize, commands: &Vec<isize>) -> usize {
		let i: isize = match self.parameter_modes[1] {
			ParameterMode::Immediate => commands[index + 1],
			_ => {
				let i = commands[index + 1] as usize;
				commands[i]
			}
		};
		println!("OUTPUT: {} index: {}, v: {}", i, index, commands.len());

		i as usize
	}
}

struct OperationReader;

impl OperationReader {
	pub fn parse_parameter_mode(n: &str) -> [ParameterMode; 2] {
		match n {
			"11" => [ParameterMode::Immediate, ParameterMode::Immediate],
			"10" => [ParameterMode::Immediate, ParameterMode::Position],
			"1" => [ParameterMode::Position, ParameterMode::Immediate],
			_ => [ParameterMode::Position, ParameterMode::Position],
		}
	}

	pub fn parse_operation(n: isize) -> Operation {
		let opc_string = n.to_string();
		let opc_length = opc_string.len();

		let (parameter_mode_input, opc) = if opc_length > 2 {
			opc_string.split_at(opc_length - 2)
		} else {
			opc_string.split_at(0)
		};

		let parameter_modes = Self::parse_parameter_mode(parameter_mode_input);
		// println!("OPC: {}", opc_string);
		let opc_command = match opc {
			"1" | "01" => OpCode::Add,
			"2" | "02" => OpCode::Mult,
			"3" | "03" => OpCode::Save,
			"4" | "04" => OpCode::Print,
			"5" | "05" => OpCode::JumpIfTrue,
			"6" | "06" => OpCode::JumpIfFalse,
			"7" | "07" => OpCode::LessThan,
			"8" | "08" => OpCode::Equals,
			"99" => OpCode::Halt,
			_ => panic!("NOOOOO {}", n)
		};

		Operation {
			parameter_modes,
			command: opc_command,
		}
	}
}

fn get_input() -> Vec<isize> {
	let input: Vec<isize> = fs::read_to_string("./day07.txt")
		.unwrap()
		.split(",")
		.map(|c| c.parse::<isize>().unwrap())
		.collect();

	input
}

fn part_two(value: isize) {
	let mut input = get_input();
	let mut index = 0;
	while index < input.len() {
		let raw_opc = &input[index];
		let operation = OperationReader::parse_operation(*raw_opc);
		index = match operation.command {
			OpCode::Add => operation.add(index, &mut input),
			OpCode::Mult => operation.mult(index, &mut input),
			OpCode::Save => operation.save(index, &mut input, value),
			OpCode::Print => operation.print(index, &input),
			OpCode::Equals | OpCode::LessThan => operation.less_than_or_equal_to(operation.command, index, &mut input),
			OpCode::JumpIfFalse => operation.jump(index, false, &mut input),
			OpCode::JumpIfTrue => operation.jump(index, true, &mut input),
			OpCode::Halt => {
				break;
			},
		};
		// println!("INDEX: {}", index);
	}
}

fn part_one(value: isize) {
	let mut input = get_input();
	let mut index = 0;
	while index < input.len() {
		let raw_opc = &input[index];
		if raw_opc == &0 {
			index += 1;
			continue;
		}

		let operation = OperationReader::parse_operation(*raw_opc);

		index = match operation.command {
			OpCode::Add => operation.add(index, &mut input),
			OpCode::Mult => operation.mult(index, &mut input),
			OpCode::Save => operation.save(index, &mut input, value),
			OpCode::Print => operation.print(index, &mut input),
			OpCode::Halt => {
				break;
			},
			_ => panic!("buyhg?")
		};
	}
}

fn run_program(input: &mut Vec<isize>, input_values: (isize, isize)) -> isize {
	let mut index = 0;
	let mut input_data = input;
	let mut output = 0;
	// reallyREALLY inelegant but fuck it
	let mut used_first_input = false;
	while index < input_data.len() {
		let raw_opc = &input_data[index];
		let operation = OperationReader::parse_operation(*raw_opc);
		index = match operation.command {
			OpCode::Add => operation.add(index, &mut input_data),
			OpCode::Mult => operation.mult(index, &mut input_data),
			OpCode::Save => {
				if used_first_input {
					operation.save(index, &mut input_data, input_values.1)
				} else {
					used_first_input = true;
					operation.save(index, &mut input_data, input_values.0)
				}
			},
			OpCode::Print => {
				output = operation.print(index, &input_data);
				index + 2
			},
			OpCode::Equals | OpCode::LessThan => operation.less_than_or_equal_to(operation.command, index, &mut input_data),
			OpCode::JumpIfFalse => operation.jump(index, false, &mut input_data),
			OpCode::JumpIfTrue => operation.jump(index, true, &mut input_data),
			OpCode::Halt => {
				println!("PROGRAM_HALTED");
				break;
			},
		};
	}
	
	output as isize
}

pub fn day_five_part_one() {
	// println!("PART_ONE");
	// part_one(1);
	println!("PART_TWO");
	// part_two(5);
	day_seven_part_one();
}

fn day_seven_part_one() {
	let mut data = [0,1,2,3,4];
	let mut permutations = Vec::new();
	heap_recursive(&mut data, |permutation| {
		permutations.push(permutation.to_vec())
	});

	let mut final_output = 0;

	for phase_settings in permutations {
		let output = get_amplifier_output(&phase_settings[..], get_input(), 0);
		if output > final_output {
			final_output = output;
		}
	}

	println!("FINAL_OUTPUT_DAY_SEVEN_PART_ONE: {}", final_output);
}

fn day_seven_part_two() {
	// let mut data = [5,6,7,8,9];
	// let mut permutations = Vec::new();
	// heap_recursive(&mut data, |permutation| {
	// 	permutations.push(permutation.to_vec())
	// });

	// let mut final_output = 0;
	// loop {
	// 	for phase_settings in permutations {
	// 		let output = get_amplifier_output(&phase_settings[..], get_input());
	// 		if output > final_output {
	// 			final_output = output;
	// 		}
	// 	}

	// println!("FINAL_OUTPUT_DAY_SEVEN_PART_ONE: {}", final_output);
}


fn get_amplifier_output_part_two(phase_settings: &[isize], input_data: Vec<isize>, initial_value: isize) -> isize {
	let mut output = initial_value;
	let mut data = input_data.clone();
	loop {
		for phase_setting in phase_settings {
			let input = (*phase_setting, output);
			println!("AMPLIFIER-DATA: {:?}", input);
			output = run_program(&mut data, input);
		}


	}

	output
}


fn get_amplifier_output(phase_settings: &[isize], input_data: Vec<isize>, initial_value: isize) -> isize {
	let mut output = initial_value;
	for phase_setting in phase_settings {
		let mut data = input_data.clone();
		let input = (*phase_setting, output);
		output = run_program(&mut data, input);
	}

	output
}

mod tests {
	use super::{
		get_amplifier_output,
	};
	#[test]
	fn test_day_seven_pattern_one() {
		let expected_value = 43210;
		let phase_settings = &[4,3,2,1,0];
		let input_data: Vec<isize> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
		let actual_value = get_amplifier_output(phase_settings, input_data, 0);

		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn test_day_seven_pattern_two() {
		let expected_value = 54321;
		let phase_settings = &[0,1,2,3,4];
		let input_data: Vec<isize> = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
		let actual_value = get_amplifier_output(phase_settings, input_data, 0);

		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn test_day_seven_pattern_three() {
		let expected_value = 65210;
		let phase_settings = &[1,0,4,3,2];
		let input_data: Vec<isize> = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
		let actual_value = get_amplifier_output(phase_settings, input_data, 0);

		assert_eq!(expected_value, actual_value);
	}

	#[test]
	fn test_day_seven_part_two_pattern_one() {
		let phase_settings = &[9,8,7,6,5];
		let input_data: Vec<isize> = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
		let expected_value = 139629729;
		let mut actual_value = get_amplifier_output(phase_settings, input_data.clone(), 0);
		while expected_value > actual_value {
			actual_value = get_amplifier_output(phase_settings, input_data.clone(), actual_value);
			println!("actual {}", actual_value);
		}

		assert_eq!(expected_value, actual_value);
	}
}