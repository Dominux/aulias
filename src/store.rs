use std::{
    fs::{File, OpenOptions},
    io,
};

use shellexpand;

const FILENAME: &str = "~/.auliases";

pub struct Store {
    file: File,
}

impl Store {
    pub fn new() -> Result<Store, io::Error> {
        let filepath = shellexpand::tilde(FILENAME).into_owned();
        let file = OpenOptions::new().write(true).create(true).open(filepath)?;
        Ok(Store { file })
    }

    pub fn alias(&mut self, key: String, value: String) -> io::Result<()> {
        Ok(())
    }

    pub fn unalias(&mut self, key: String, value: String) -> io::Result<()> {
        Ok(())
    }
}
