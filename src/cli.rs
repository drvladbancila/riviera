use std::io::Write;

pub struct CLI {
    output_buffer: String,
    input_buffer: String,
}

#[allow(dead_code)]
impl CLI {
    pub fn new() -> CLI {
        CLI {
            output_buffer: String::new(),
            input_buffer: String::new()
        }
    }

    pub fn show_output(&mut self) {
        print!("{}", self.output_buffer);
        let _ = std::io::stdout().flush();
        self.output_buffer.clear();
    }

    pub fn get_input(&mut self) {
        match std::io::stdin().read_line(&mut self.input_buffer) {
            Ok(_a) => (),
            Err(err) => panic!("Could not get input: {}", err),
        }
    }

    pub fn write_byte(&mut self, value: u8) {
        self.output_buffer.push(value as char);
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.input_buffer.len() > 0 {
            self.input_buffer.remove(0).try_into().unwrap()
        } else {
            0
        }
    }
}