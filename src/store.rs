use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

use itertools::Itertools;
use regex::Regex;
use shellexpand;

const FILENAME: &str = "~/.auliases";

pub struct Store;

impl Store {
    pub fn alias(key: String, value: String) -> io::Result<()> {
        let mut list = Self::list()?;
        list.insert(key.clone(), value);
        Self::write(list)?;
        Ok(())
    }

    pub fn unalias(key: String) -> io::Result<()> {
        let mut list = Self::list()?;
        list.remove(&key);
        Self::write(list)?;
        Ok(())
    }

    /// List all aliases
    pub fn list() -> io::Result<HashMap<String, String>> {
        let file = Self::open_file(true, false)?;
        let re_alias = Regex::new(r"alias ").expect("Wrong regex");
        let re_equal_sign = Regex::new(r"=").expect("Wrong regex");

        let mut list = HashMap::new();

        // TODO: add adequate parsing/validating
        BufReader::new(&file).lines().for_each(|line| {
            let line = &line.unwrap();
            let alias = re_alias.splitn(line, 2).last();

            let (key, value) = match alias {
                Some(alias) => match re_equal_sign.splitn(alias, 2).collect_tuple() {
                    Some(key_value_tuple) => key_value_tuple,
                    None => {
                        return;
                    }
                },
                None => {
                    return;
                }
            };
            list.insert(String::from(key), String::from(value));
        });

        Ok(list)
    }

    /// Write list of aliases into file
    fn write(list: HashMap<String, String>) -> io::Result<()> {
        let mut file = Self::open_file(false, true)?;

        // Contain the file with lines
        for (key, value) in list {
            let line = format!("alias {key}={value}\n", key = key, value = value);
            file.write(line.as_bytes())?;
        }
        Ok(())
    }

    /// Open file with needed permissions
    fn open_file(to_read: bool, to_write: bool) -> io::Result<File> {
        let filepath = shellexpand::tilde(FILENAME).into_owned();
        OpenOptions::new()
            .read(to_read)
            .write(to_write)
            .truncate(to_write)
            .create(to_write)
            .open(filepath)
    }
}
