use std::io;
use std::io::prelude::*;
use std::fs::{self, File};

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

	pub fn write(&mut self, filedate: Vec<u8>) {
		self.date = filedate.clone();
	}
	
    pub fn save(&self) -> io::Result<()> {
        let filename = &self.name;
        let mut file = File::create(&*filename)?;
        file.write(self.date.as_slice())?;
        Ok(())
    }
	
	pub fn date(&self) -> Vec<u8> {
		self.date.clone()
	}
	
	pub fn date_as_string(&self) -> String {
		String::from_utf8(self.date()).unwrap()
	}
	
	pub fn kill(&self) -> io::Result<()> {
		fs::remove_file(&*self.name())?;
		Ok(())
	}
}