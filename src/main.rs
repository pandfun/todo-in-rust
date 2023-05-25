use std::fs::{self};

fn main()
{
	let file_path = "src/data.txt";

	match fs::read_to_string(file_path) {
		Ok(buffer) => {
			parse_data(buffer);
		},
		Err(error) => {
			panic!("Error reading file: {:?}", error);
		}
	}

}

fn parse_data(buff: String) {

	let buff = buff.as_bytes();
	let mut flag = 0;
	/* Flag Values

		0 	No job/free
		1	Doing some job
	*/

	println!("\nTODO LIST:");
	for it in buff {

		if flag == 0 && *it == b'p' {
			flag = 1;
			print!("□ ");
			continue;
		}

		if flag == 0 && *it == b'x' {
			flag = 1;
			print!("☑️ ");
			continue;
		}

		if *it == b'\n' {
			print!("\n");
			flag = 0;
		} else {
			print!("{}", *it as char);
		}
 	}


}