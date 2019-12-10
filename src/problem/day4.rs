use std::time::Instant;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Num([u8; 6]);

impl Num {
    fn from_i32(input: i32) -> Self {
        let d0 = ((input / 100_000) % 10) as u8;
        let d1 = ((input / 10_000) % 10) as u8;
        let d2 = ((input / 1_000) % 10) as u8;
        let d3 = ((input / 100) % 10) as u8;
        let d4 = ((input / 10) % 10) as u8;
        let d5 = (input % 10) as u8;

        Self {
            0: [d0, d1, d2, d3, d4, d5],
        }
    }

    fn inc(&mut self) {
        for i in (0..6).into_iter().rev() {
            self.0[i] += 1;
            if self.0[i] != 10 {
                return;
            }

            if i == 0 {
                self.0[i] = 0;
            } else {
                self.0[i] = self.0[i - 1];
            }
        }
    }
}

fn is_2_digit_same(input: &Num) -> bool {
    (0..5).any(|i| input.0[i] == input.0[i + 1])
}

fn is_2_digit_same_advanced(input: &Num) -> bool {
    let input = input.0;

    (0..5).any(|i| match i {
        0 => (input[0] == input[1]) && (input[0] != input[2]),
        4 => (input[4] == input[5]) && (input[4] != input[3]),
        n => (input[n] == input[n + 1]) && (input[n] != input[n - 1]) && (input[n] != input[n + 2]),
    })
}

fn is_increase(input: &Num) -> bool {
    (0..5).all(|i| input.0[i] <= input.0[i + 1])
}


fn has_adjacent_units(number: &[u8]) -> bool {
	let mut range = 0;
	let mut met_criteria = false;
	let length = number.len();
	while range < length - 1 {
		if number[range] == number[range + 1] {
			met_criteria = true;
			let num = number[range];
			let mut subrange = range;
			let mut occurrences = 1;
			range += 1;
			while subrange < length - 1 && num == number[subrange + 1]  {
				occurrences += 1;
				subrange += 1;
			}
			if occurrences == 3 || occurrences == 5 {
				met_criteria = false;
				break;
			}
			// println!("for {:?} OCCURRENCES: {} for {}", number, occurrences, num);
		}

		range += 1;
	}

	met_criteria
}

// fn has_uneven_groups(number: &[u8]) -> bool {
// 	let mut is_uneven = false;
	

// 	is_uneven
// }

fn numbers_are_ascending(number: &[u8]) -> bool {
  let mut index = 0;
  let mut is_ascending = false;
  
  loop {
    if index + 1 == 6 {
		// println!("LESSER {:?}", number);
      break;
    }
    let lesser = number[index];
	let greater = number[index + 1];
	// if number[index + 1] == 48 {
	// 	println!("KJDFBHKASJFHSKJDFHS");
	// }
    is_ascending = lesser <= greater;
    index += 1;

    if !is_ascending {
      break;
    }
  }

  is_ascending
}

fn meets_criteria(number: &[u8]) -> bool {
	numbers_are_ascending(number) && has_adjacent_units(number)
}

pub fn day_4_part_one() -> usize {
	let mut possibilities = 0usize;
	let mut low = Num::from_i32(183564);
	let high = Num::from_i32(657474);
	let mut task_a = 0;
    let mut task_b = 0;

    loop {
        if is_increase(&low) {
            if is_2_digit_same(&low) {
                task_a += 1;
            }

            if is_2_digit_same_advanced(&low) {
                task_b += 1;
            }
        }

        low.inc();

        if low > high {
            break;
        }
    }
	// for i in range {
	// 	let string = i.to_string().into_bytes();
	// 	if !meets_criteria(&string) {
	// 		println!("{}", i);
	// 	}
	// 	// if meets_criteria(&string) {
	// 	// 	possibilities += 1;
	// 	// }
	// }

	// println!("TOTAL: {}", possibilities);
	println!("{}", task_b);
	possibilities
}

#[cfg(test)]
mod tests {
	use super::{
		has_adjacent_units,
		numbers_are_ascending,
		meets_criteria,
	};
  	#[test]
  	fn test_adjacent() {
		assert_eq!(has_adjacent_units(b"112233"), true);
		assert_eq!(has_adjacent_units(b"123456"), false);
		assert_eq!(has_adjacent_units(b"135679"), false);
		assert_eq!(has_adjacent_units(b"128770"), true);
	}

	#[test]
	fn test_meets_criteria() {
		assert_eq!(meets_criteria(b"114444"), true);
		assert_eq!(meets_criteria(b"111122"), true);
		assert_eq!(meets_criteria(b"128777"), false);
		assert_eq!(meets_criteria(b"588889"), true);
		assert_eq!(meets_criteria(b"411111"), false);
		// assert_eq!(meets_criteria(b"445567"), false);
	}

  	#[test]
  	fn test_ascending() {
		assert_eq!(numbers_are_ascending(b"135679"), true);
		assert_eq!(numbers_are_ascending(b"123256"), false);
		assert_eq!(numbers_are_ascending(b"123353"), false);
		assert_eq!(numbers_are_ascending(b"123499"), true);
	}
}