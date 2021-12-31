use aulias::args_parser::get_arg;
use aulias::store::Store;

fn main() {
	let key = get_arg().unwrap();
	println!("Finish: {:?}", Store::unalias(key));
}

