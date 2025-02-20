fn main(){
	let message = "First";
	print_message(message)
}

fn print_message(text: &str){ // Pattern of funtion naming use xxx_xxx
	println!("{}", text);
}

// First
