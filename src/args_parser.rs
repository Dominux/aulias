use std::env;

use itertools::Itertools;
use regex::Regex;

// TODO: separate func into args reader and args parser/validator
pub fn get_key_value_arg() -> Result<(String, String), ()> {
    let regex_separator: Regex = Regex::new(r"=").expect("Wrong regex separator!");

    // TODO: create resolving of several args
    let arg = get_arg()?;

    let key_value: Vec<&str> = regex_separator.splitn(arg.as_str(), 2).collect();
    match key_value.len() {
        2 => Ok(key_value
            .into_iter()
            .map(|x| String::from(x))
            .collect_tuple()
            .unwrap()),
        _ => {
            println!("{}", key_value[0]);
            Err(())
        }
    }
}

pub fn get_arg() -> Result<String, ()> {
    let mut args = env::args();
    args.next().ok_or(())?;
    args.next().ok_or(())
}
