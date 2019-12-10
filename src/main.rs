use nom::{alt, char, named, map, pair, parse_to};
use std::fs;
use failure::Error;

pub mod problem;


#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Instruction {
    pub direction: Direction,
    pub value: u32,
}

named!(
    pub dir<&[u8], char>,
    alt!(
        char!('U') |
        char!('D') |
        char!('L') |
        char!('R')
    )
);

named!(
    pub instruction<&[u8], Instruction>,
    map!(
        pair!(
            dir,
            parse_to!(u32)
        ),
        |(c, value)| {
            let direction = match c {
                'U' => Direction::Up,
                'L' => Direction::Left,
                'R' => Direction::Right,
                'D' => Direction::Down,
                _ => unreachable!("unpossible")
            };

            Instruction {
                direction, value
            }
        }
    )
);

// named!(
//     pub instruction_parser<&[u8], Vec<Instruction>>,
//     map!(
//         instruction,
//         eat_separator!(b",")
//     )
// );

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
struct Pathway {
    from: Point,
    to: Point,
    direction: Direction,
}

fn old_stuff() -> Result<(), Error> {
    let input = fs::read_to_string("./day3.txt")?;
    let mut lines = input.lines();
    let first_line: Vec<Instruction> = lines.next().unwrap().split(",").map(|i| {
        instruction(i.as_bytes()).unwrap().1
    }).collect();
    let second_line: Vec<Instruction> = lines.next().unwrap().split(",").map(|i| {
        instruction(i.as_bytes()).unwrap().1
    }).collect();
    // let mut intersection_points: Vec<Point> = vec![];
    
    let mut first_path: Vec<Pathway> = vec![];
    let mut second_path: Vec<Pathway> = vec![];
    let mut cursor = Point { x: 0, y: 0 };
    let mut xmax = 0;
    let mut ymax = 0;
    for p in first_line {
        let destination = match p.direction {
            Direction::Down => Point { x: cursor.x, y: cursor.y - p.value as i32 },
            Direction::Up => Point { x: cursor.x, y: cursor.y + p.value as i32 },
            Direction::Left => Point { x: cursor.x - p.value as i32, y: cursor.y },
            Direction::Right => Point { x: cursor.x + p.value as i32, y: cursor.y },
        };

        first_path.push(Pathway { from: cursor, to: destination, direction: p.direction });
        if destination.x > xmax {
            xmax = destination.x;
        }
        cursor = destination;
    }

    println!("{:#?}", xmax);

    cursor = Point { x: 0, y: 0 };

    for p in second_line {
        let destination = match p.direction {
            Direction::Down => Point { x: cursor.x, y: cursor.y - p.value as i32 },
            Direction::Up => Point { x: cursor.x, y: cursor.y + p.value as i32 },
            Direction::Left => Point { x: cursor.x - p.value as i32, y: cursor.y },
            Direction::Right => Point { x: cursor.x + p.value as i32, y: cursor.y },
        };

        second_path.push(Pathway { from: cursor, to: destination, direction: p.direction });
        cursor = destination;
    }

    // for fpath in first_path {
    //     for spath in second_path.clone() {
    //         println!("{:#?}", intersection_point(&fpath, &spath))
    //     }
    // }




    // if let Some(first_line) = lines.next() {
    //     first_line
    // } else {
    //     unreachable!()
    // };

    // if let Some(second_line) = lines.next() {
    //     second_line
    // } else {
    //     unreachable!()
    // };

    
    // for line in input.lines() {
    //     let bytes = line.split(",").map(|i| instruction(i.as_bytes()));
    //     for byte in bytes {
    //         println!("{:#?}", byte);
    //     }
    // }

    Ok(())
}

fn main() -> Result<(), Error> {
    // problem::day4::day_4_part_one();
    problem::day05::day_five_part_one();
    Ok(())
}

fn intersection_point(first_line: &Pathway, second_line: &Pathway) -> Option<Point> {
    // can't be moving on the same axis
    // if the first line is not changing on the x axis, but the second line is...
    
    // if first_line.from.0 == first_line.to.0 
    // imagining first_line = start (5, 3), end (5, 8)
    // second_line = (1, 1), (1, 10)
    // if first_line.from.0 == first_line.to.0

    match first_line.direction {
        Direction::Up => {
            // if second_line.from.x == second_line.to.x && second_line.to.y > first_line.from.y && second_line.from.y < 
            match second_line.direction {
                Direction::Left => {
                    if second_line.from.y > first_line.from.y && second_line.from.y < first_line.to.y {
                        Some(
                            Point {
                                x: second_line.from.x,
                                y: second_line.from.y - first_line.to.y,
                            }
                        )
                    } else {
                        None
                    }
                },
                _ => None
            }
        },
        _ => None
        // Direction::Up | Direction::Down => {
        //     // first line is running along the Y axis e.g. 10, 5 to 10, 10 (Y IS CHANGING, NOT X)
        //     // so figure out if the SECOND LINE's Y axis sits somewhere in the middle between the two points Y values
        //     // 
        //     match second_line.direction {
        //         Direction::Left | Direction::Right => {
        //             if second_line.
        //         }
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        dir,
        instruction,
    };
    #[test]
    fn parse_direction() {
        println!("{:#?}", dir(b"L435"));
        println!("{:#?}", dir(b"l435"));
        println!("{:#?}", dir(b"435"));
    }

    #[test]
    fn parse_instruction() {
        println!("{:#?}", instruction(b"L435"));
        println!("{:#?}", instruction(b"L435,R10"));
    }
}



















// fn extract_file_contents(file_path: &str) -> Vec<isize> {
//     let mut lines: Vec<isize> = vec![];

//     let contents = match fs::read_to_string(file_path) {
//         Ok(file) => file,
//         Err(e) => panic!("file unable to be read, {:?}", e)
//     };

//     for line in contents.lines() {
//         let num = match line.parse::<isize>() {
//             Ok(i) => i,
//             Err(_) => panic!("NUM NOT PARSED")
//         };

//         lines.push(num);
//     }

//     lines
// }

// /*
// At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.

// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.

// For example:

// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
// For a mass of 1969, the fuel required is 654.
// For a mass of 100756, the fuel required is 33583.
// The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.

// What is the sum of the fuel requirements for all of the modules on your spacecraft?


// Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require negative fuel should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead handled by wishing really hard, which has no mass and is outside the scope of this calculation.

// So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:

// A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
// At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
// The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
// What is the sum of the fuel requirements for all of the modules on your
// */ 

// fn day_one() {
//     let input = extract_file_contents("./day1.txt");

//     day_one_answer(vec![12, 14, 1969, 100756], true);
//     day_one_answer(input, false);
// }

// fn fuel_cost(mass: isize) -> isize {
//     mass / 3isize - 2
// }

// fn calculate_fuel_requirement(mass: isize) -> isize {
//     let mut fuel = fuel_cost(mass);
//     if fuel <= 0 {
//         return 0
//     }

//     let mut cost = fuel;

//     while fuel >= 0 {
//         fuel = fuel_cost(fuel);
//         if fuel >= 0 {
//             cost += fuel;
//         }
//     }

//     cost
// }

// fn day_one_answer(input: Vec<isize>, debug: bool) {
//     let mut total_requirement = 0isize;
//     let mut fuel_costs = 0isize;
//     for mass in input {
//         let req = fuel_cost(mass);
//         fuel_costs += calculate_fuel_requirement(req);

//         if debug {
//             println!("DEBUG: {}, fuel: {}", req, fuel_costs);
//         }
//         total_requirement += req;
//     }


//     if debug {
//         println!("\n\n\nDEBUGDEBUGDEBUG\n\n\nBefore Fuel Cost: {}\nFuel Cost: {}\nGrandTotal: {}\n", total_requirement, fuel_costs, fuel_costs + total_requirement);
//     } else {
//         println!("TOTAL: {}", total_requirement + fuel_costs);
//     }
// }

// // Encountering an unknown opcode means BAD

// enum OpCode {
//     Halt,
//     // Opcode 1 adds together numbers read from two positions and
//     // stores the result in a third position. The three integers
//     // immediately after the opcode tell you these three positions
//     //  - the first two indicate the positions from which you should read the input values,
//     // and the third indicates the position at which the output should be stored.
//     Add(usize, usize, usize), // OPCODE 1
//     // OPCODE 2, same as 1 but multiplies
//     Mult(usize, usize, usize)
// }

// /*
// Once you have a working computer, the first step is to restore the gravity
// assist program (your puzzle input) to the "1202 program alarm" state it
// had just before the last computer caught fire. To do this, before running the program,

// replace position 1 with the value 12 and replace position 2 with the value 2.
// What value is left at position 0 after the program halts?
// */
// fn day_two() {
//     let contents = fs::read_to_string("./day2.txt");
//     let input = match contents {
//         Ok(file) => file,
//         Err(_) => panic!("DOINK")
//     };

//     // let input = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
//     let opcodes_str = &input.replace(",", "\n");
//     let mut opcodes: Vec<usize> = vec![];
//     for line in opcodes_str.lines() {
//         let num = match line.parse::<usize>() {
//             Ok(i) => i,
//             Err(_) => panic!("NUM NOT PARSED")
//         };

//         opcodes.push(num);
//     }

//     let mut opcode_index = 0;
//     // let mut a_index = 0;
//     // let mut b_index = 0;
//     // let mut c_index = 0;

//     if let Some(num) = opcodes.get_mut(1) {
//         *num = 39;
//     };

//     if let Some(num) = opcodes.get_mut(2) {
//         *num = 51;
//     };

//     loop {
//         if opcode_index >= opcodes.len() {
//             break;
//         }
//         let opc = opcodes[opcode_index];
//         let action = match opc {
//             1 => OpCode::Add(opcodes[opcode_index + 1], opcodes[opcode_index + 2], opcodes[opcode_index + 3]),
//             2 => OpCode::Mult(opcodes[opcode_index + 1], opcodes[opcode_index + 2], opcodes[opcode_index + 3]),
//             99 => OpCode::Halt,
//             _ => panic!("invalid")
//         };

//         let (new_value, position) = get_result(action, &opcodes);
//         println!("SAJDghAKSJdhKSDJHD: {}", new_value);

//         if new_value == 0 && position == 0 {
//             println!("BREAK {}", opcode_index);
//             break;
//         }

//         if let Some(num) = opcodes.get_mut(position) {
//             *num = new_value;
//         }

//         opcode_index += 4;
//     }

//     println!("FINAL {}", opcodes[0]);
// }

// fn get_result(operation: OpCode, values: &Vec<usize>) -> (usize, usize) {
//     println!("{:?}", values);
//     match operation {
//         OpCode::Add(x, y, v) => {
//             println!("ADD: x: {}, y: {}, v: {}", x, y, v);
//             (values[x] + values[y], v)
//         },
//         OpCode::Mult(x, y, v) => {
//             println!("MULT: x: {}, y: {}, v: {}", x, y, v);
//             (values[x] * values[y], v)
//         },
//         _ => (0, 0)
//     }
// }

// // enum Direction {
// //     Up(usize),
// //     Left(usize),
// //     Right(usize),
// //     Down(usize)
// // }

// // fn parse_instruction_set(instruction: &str) -> Direction {
// //     let (direction, amount): (Vec<char>, Vec<char>) = instruction.chars().partition(|&c| c.is_digit(10));
// //     let dir = direction[0];

// //     let s = amount
// //     match dir {
// //         'U' => Direction::Up()
// //     }


    


// //     // let set = instruction.chars();
// //     // let dir = set.nth(0);
// //     // let steps = set.take_while(|n| n.is_digit(10)).collect();

    
// // }

// // fn parse_wires(input: String) -> (Vec<Direction>, Vec<Direction>) {
// //     let mut first_wire: Vec<Direction> = vec![];
// //     let mut second_wire: Vec<Direction> = vec![];

// //     for line in input.lines() {
// //         let sets: Vec<Direction> = line
// //             .split(",")
// //             .map(|n| parse_instruction_set(n))
// //             .collect();
// //         // let instruction = parse_instruction_set(instruction: &str)
// //     }

// //     (first_wire, second_wire)
// // }

// // fn day_three() {
// //     let content = fs::read_to_string("./day3.txt");

// //     let input = match content {
// //         Ok(file) => file,
// //         Err(_) => panic!("you know the dril")
// //     };


// // }