pub const DIGITO: i32 = 1;
pub const OPERATOR: i32 = 3;

pub struct LexScanner {
    pub content: Vec<char>,
    pub state: i32,
    pub pos: usize
}

impl LexScanner {
    /* pub fn prepare_scanner(&mut self, file: String) {
        self.content = file.chars().collect();
    } */

    pub fn next_token(&mut self) {
        self.state = 0;
        self.pos = 0;

        let mut c: char;

        loop {
            c = self.content[self.pos];

            match self.state {
                0 => {
                    match self.is_digit(c) {
                        true => self.state = DIGITO,
                        false => match self.is_operator(c) {
                            true => self.state = OPERATOR,
                            false => ()
                        }
                    };
                }
                _ => ()
            };

            self.pos += 1;

            if self.pos == self.content.len() { break; }
        }
    }

    pub fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9'=> true,
            _ => false
        }
    }

    pub fn is_operator(&self, c: char) -> bool {
        match c {
            '+'|'-'|'*'|'/'|'^' => true,
            _ => false
        }
    }
}