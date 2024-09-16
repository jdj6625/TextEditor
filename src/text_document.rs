use std::fs::{File, OpenOptions};
use std::io::{read_to_string, Read};
struct TextDocument<'a> {
    init_buffer: bool,
    data: String,
    length: usize,
}

impl TextDocument{
    fn init(&mut self, file_name: &str) -> std::io::Result<()>
    {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name);
        file.read_to_string(&mut self.data)?;
        Ok(())
    }
    fn get_line<'a>(line_number: usize, char_buffer: &'a char, line_length: usize) -> u64{
        line
    }
    fn line_count(){

    }

}