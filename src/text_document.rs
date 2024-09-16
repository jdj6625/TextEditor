use std::fs::OpenOptions;
use std::io::Read;
struct TextDocument<'a> {
    init_buffer: bool,
    line_buffer: Vec<&'a str>,
    length: usize,
}

impl TextDocument {
    // Constructor
    fn init(&mut self, file_name: &str) -> std::io::Result<()>
    {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data)?;
        let lines = data.lines();
        for line in lines
        {
            self.line_buffer.push(line)
        }
        self.length = self.line_buffer.len();
        Ok(())
    }

    fn get_line<'a>(&self, line_number: usize) -> &'a str {
        self.line_buffer[line_number]
    }
}