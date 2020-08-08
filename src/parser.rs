/// Markdown parser
/// Contains the position and input fields.
/// * position
/// * input: the string on the current line we are parsing
///
/// We use the position property to find out where in our string we have white-space
/// and then we split our string into small token characters and parse each as we get
/// them.
///
/// If we find a string with a `#` sign, we wrap it in a `<h1>` tag.
struct Parser {
    position: usize,
    input: String,
}

impl Parser {

    /// Checks to see whether or not the position of the character we
    /// are reading is greater-than or equal to the actual string length
    fn end_of_line(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Check to see if at te current position the input starts with a
    /// specific character
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    /// Outputs the character at the next position of iterator
    fn next_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    /// Advances through our string and return the next position
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.position..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        let (next_position, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_position;
        current_char
    }

    /// Output string if the character is not at end of line
    /// and pushes the next char to the result string
    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.end_of_line() && condition(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Consume white-space
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace());
    }
}