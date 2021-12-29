use std::fs::{OpenOptions, File};

mod aulias;
mod store;

fn main() {
	let (key, value) = aulias::get_arg().expect("msg");
	let mut store = store::Store::new().expect("mslg");
	store.alias(key, value);
}
