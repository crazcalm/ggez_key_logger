pub struct LogBuf {
    size: usize,
    stack: Vec<String>,
}

impl LogBuf {
    pub fn new(size: usize) -> LogBuf {
        LogBuf {
            size,
            stack: Vec::new(),
        }
    }

    pub fn add(&mut self, character: char) {
        // If stack is full, remove a char
        if self.stack.len() >= self.size {
            let _ = self.stack.remove(0);
        }

        self.stack.push(character.to_string());
    }

    pub fn delete(&mut self) {
        if self.stack.len() > 0 {
            let _ = self.stack.pop().expect("Unable to pop off the stack");
        }
    }

    pub fn get_text(&self) -> String {
        let mut text = String::new();

        for character in self.stack.iter() {
            text.push_str(character);
        }

        text
    }
}

#[cfg(test)]
mod tests {
    use super::LogBuf;

    #[test]
    fn test_add() {
        let size = 5;
        let mut log_buf = LogBuf::new(size);

        let test_chars = vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];

        let expected = vec!['w', 'o', 'r', 'l', 'd'];
        for character in test_chars.iter() {
            log_buf.add(character.clone());
        }

        for index in 0..size {
            assert_eq!(expected[index].to_string(), log_buf.stack[index]);
        }
    }

    #[test]
    fn test_delete() {
        let test_chars = vec!['h', 'e', 'l', 'l', 'o'];
        let mut log_buf = LogBuf::new(test_chars.len());

        for character in test_chars.iter() {
            log_buf.add(character.clone());
        }

        // 3 deletes
        log_buf.delete();
        log_buf.delete();
        log_buf.delete();

        assert_eq!(log_buf.stack.len(), 2);
        for index in 0..log_buf.stack.len() {
            assert_eq!(log_buf.stack[index], test_chars[index].to_string());
        }

        // 3 more deletes to cover delete while
        // empty case
        log_buf.delete();
        log_buf.delete();
        log_buf.delete();

        assert_eq!(log_buf.stack.len(), 0);
    }
}
