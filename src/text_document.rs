use std::fs::OpenOptions;
use std::io::Read;

pub struct TextDocument {
    pub(crate) line_buffer: Vec<String>,
    pub(crate) length: usize,
}

impl TextDocument {
    pub fn init(&mut self, file_name: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data)?;
        let lines = data.lines();
        for line in lines {
            self.line_buffer.push(line.to_string());
        }
        self.length = self.line_buffer.len();
        Ok(())
    }

    pub fn get_line(&self, line_number: usize) -> String {
        self.line_buffer.get(line_number).unwrap().to_string()
    }

    pub fn line_count(&self) -> usize {
        self.length
    }
}
