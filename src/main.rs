mod args_parser;
mod store;

fn main() {
	let (key, value) = args_parser::get_arg().expect("msg");
	println!("Finish: {:?}", store::Store::alias(key.clone(), value));
	println!("Finish: {:?}", store::Store::unalias(key));
}
