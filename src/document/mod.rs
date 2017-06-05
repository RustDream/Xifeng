use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct Document {
    name: String,
    date: Vec<u8>,
}

impl Document {
    pub fn new(filename: &str) -> Self {    
        Document {
            name: filename.to_string(),
            date: Vec::new(),
        }
    }
    
    pub fn read(&mut self) -> io::Result<()> {
        let filename = &self.name;
        let mut file = File::open(&*filename)?;
        file.read_to_end(&mut self.date)?;
        Ok(())
    }
    
    pub fn rename(&mut self, filename: &str) {
        self.name = filename.to_string();
    }
    
    pub fn name(&self) -> String {
        self.name.clone()
    }
    
    pub fn save(&self) -> io::Result<()> {
        let filename = &self.name;
        let mut file = File::create(&*filename)?;
        file.write(self.date.as_slice())?;
        Ok(())
    }
}