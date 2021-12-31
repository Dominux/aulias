use aulias::args_parser::get_arg;
use aulias::store::Store;

fn main() {
	let (key, value) = get_arg().expect("msg");
	println!("Finish: {:?}", Store::alias(key.clone(), value));
}
