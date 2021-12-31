use aulias::args_parser::get_key_value_arg;
use aulias::store::Store;

fn main() {
    let (key, value) = get_key_value_arg().unwrap();
    println!("Finish: {:?}", Store::alias(key, value));
}
