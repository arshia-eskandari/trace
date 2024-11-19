pub trait StringExt {
    fn pad_start(&self, pad_count: usize, pad_char: Option<char>) -> String;
    fn pad_end(&self, pad_count: usize, pad_char: Option<char>) -> String;
    fn pad_start_to_length(&self, length: usize, pad_char: Option<char>) -> String;
    fn pad_end_to_length(&self, length: usize, pad_char: Option<char>) -> String;
}

impl StringExt for String {
    fn pad_start(&self, pad_count: usize, pad_char: Option<char>) -> String {
        let padding = pad_char.unwrap_or(' ').to_string().repeat(pad_count);
        format!("{}{}", padding, self)
    }

    fn pad_end(&self, pad_count: usize, pad_char: Option<char>) -> String {
        let padding = pad_char.unwrap_or(' ').to_string().repeat(pad_count);
        format!("{}{}", self, padding)
    }

    fn pad_start_to_length(&self, length: usize, pad_char: Option<char>) -> String {
        if length <= self.len() {
            self.to_string()
        } else {
            let pad_length = length - self.len();
            let padding = pad_char.unwrap_or(' ').to_string().repeat(pad_length);
            format!("{}{}", padding, self) 
        }
    }

    fn pad_end_to_length(&self, length: usize, pad_char: Option<char>) -> String {
        if length <= self.len() {
            self.to_string()
        } else {
            let pad_length = length - self.len();
            let padding = pad_char.unwrap_or(' ').to_string().repeat(pad_length);
            format!("{}{}", self, padding)
        }
    }
}
