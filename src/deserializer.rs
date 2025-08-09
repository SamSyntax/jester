use crate::serializer::JsonVal;

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.next();
        }
    }

    pub fn parse_value(&mut self) -> Result<JsonVal, String> {
        self.skip_whitespace();
        match self.peek() {
            Some('n') => self.parse_null(),
            Some('t') | Some('f') => self.parse_bool(),
            Some('"') => self.parse_string(),
            Some('0'..='9') => self.parse_number(),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            _ => Err("Unexpected character".to_string()),
        }
    }

    fn parse_null(&mut self) -> Result<JsonVal, String> {
        if self.input[self.pos..].starts_with("null") {
            self.pos += 4;
            Ok(JsonVal::Null)
        } else {
            Err("Invalid null".to_string())
        }
    }

    fn parse_bool(&mut self) -> Result<JsonVal, String> {
        if self.input[self.pos..].starts_with("true") {
            self.pos += 4;
            Ok(JsonVal::Bool(true))
        } else if self.input[self.pos..].starts_with("false") {
            self.pos += 5;
            Ok(JsonVal::Bool(false))
        } else {
            Err("Invlid boolean".to_string())
        }
    }

    fn parse_string(&mut self) -> Result<JsonVal, String> {
        self.next();
        let mut result = String::new();
        while let Some(ch) = self.next() {
            match ch {
                '"' => return Ok(JsonVal::String(result)),
                '\\' => {
                    if let Some(esc) = self.next() {
                        match esc {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            '/' => result.push('/'),
                            'b' => result.push('\u{0008}'),
                            'f' => result.push('\u{000C}'),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'u' => {
                                let mut hex = String::new();
                                for _ in 0..4 {
                                    if let Some(h) = self.next() {
                                        hex.push(h);
                                    } else {
                                        return Err("Invlid unicode escape".to_string());
                                    }
                                }
                                if let Ok(code_point) = u16::from_str_radix(&hex, 16) {
                                    if let Some(c) = std::char::from_u32(code_point as u32) {
                                        result.push(c);
                                    } else {
                                        return Err("Invalid unicode code point".to_string());
                                    }
                                } else {
                                    return Err("Invalid unicode escape".to_string());
                                }
                            }
                            _ => return Err(format!("Invalid escape sequence: \\{}", esc)),
                        }
                    } else {
                        return Err("Unterminated escape sequence".to_string());
                    }
                }
                _ => result.push(ch),
            }
        }
        Err("Unterminated string".to_string())
    }

    fn parse_number(&mut self) -> Result<JsonVal, String> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.next();
        }
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.next();
        }

        let mut is_float = false;
        if self.peek() == Some('.') {
            is_float = true;
            self.next();
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.next();
            }
        }

        if matches!(self.peek(), Some('e') | Some('E')) {
            is_float = true;
            self.next();
            if matches!(self.peek(), Some('+') | Some('-')) {
                self.next();
            }
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.next();
            }
        }
        let num_str = &self.input[start..self.pos];
        if is_float {
            num_str
                .parse::<f64>()
                .map(JsonVal::Float)
                .map_err(|_| "Invalid float".to_string())
        } else {
            num_str
                .parse::<usize>()
                .map(JsonVal::Number)
                .map_err(|_| "Invalid float".to_string())
        }
    }
    fn parse_array(&mut self) -> Result<JsonVal, String> {
        self.next();
        let mut elements = Vec::new();
        loop {
            self.skip_whitespace();
            if let Some(']') = self.peek() {
                self.next();
                break;
            }
            elements.push(self.parse_value()?);
            self.skip_whitespace();
            if let Some(',') = self.peek() {
                self.next();
            }
        }
        Ok(JsonVal::Array(elements))
    }
    fn parse_object(&mut self) -> Result<JsonVal, String> {
        self.next();
        let mut map = std::collections::HashMap::new();
        loop {
            self.skip_whitespace();
            if let Some('}') = self.peek() {
                self.next();
                break;
            }

            if let JsonVal::String(key) = self.parse_string()? {
                self.skip_whitespace();
                if self.next() != Some(':') {
                    println!("Err on key: {}", key);
                    return Err("Unexpected ':'".to_string());
                }
                let value = self.parse_value()?;
                map.insert(key, value);
                self.skip_whitespace();
                if let Some(',') = self.peek() {
                    self.next();
                }
            } else {
                return Err("Expected string key".to_string());
            }
        }
        Ok(JsonVal::Object(map))
    }
}
