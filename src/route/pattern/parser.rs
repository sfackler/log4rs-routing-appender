use std::iter::Peekable;
use std::str::CharIndices;

pub enum Piece<'a> {
    Text(&'a str),
    Argument { name: &'a str, args: Vec<&'a str> },
    Error(&'static str),
}

pub struct Parser<'a> {
    pattern: &'a str,
    it: Peekable<CharIndices<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(pattern: &'a str) -> Parser<'a> {
        Parser {
            pattern: pattern,
            it: pattern.char_indices().peekable(),
        }
    }

    fn consume(&mut self, ch: char) -> bool {
        match self.it.peek() {
            Some(&(_, c)) if c == ch => {
                self.it.next();
                true
            }
            _ => false,
        }
    }

    fn argument(&mut self) -> Piece<'a> {
        if !self.consume('{') {
            return Piece::Error("expected `{`");
        }
        let name = self.name();
        let args = match self.args() {
            Ok(args) => args,
            Err(e) => return Piece::Error(e),
        };
        if !self.consume('}') {
            return Piece::Error("expected `}`");
        }
        Piece::Argument {
            name: name,
            args: args,
        }
    }

    fn name(&mut self) -> &'a str {
        let start = match self.it.peek() {
            Some(&(pos, ch)) if ch.is_alphabetic() => {
                self.it.next();
                pos
            }
            _ => return "",
        };

        loop {
            match self.it.peek() {
                Some(&(_, ch)) if ch.is_alphanumeric() => {
                    self.it.next();
                }
                Some(&(end, _)) => return &self.pattern[start..end],
                None => return &self.pattern[start..],
            }
        }
    }

    fn args(&mut self) -> Result<Vec<&'a str>, &'static str> {
        let mut args = vec![];
        while let Some(arg) = self.arg()? {
            args.push(arg);
        }
        Ok(args)
    }

    fn arg(&mut self) -> Result<Option<&'a str>, &'static str> {
        if !self.consume('(') {
            return Ok(None);
        }

        let start = match self.it.next() {
            Some((_, ')')) => return Ok(Some("")),
            Some((pos, _)) => pos,
            None => return Err("Expected `)`"),
        };

        loop {
            match self.it.next() {
                Some((pos, ')')) => return Ok(Some(&self.pattern[start..pos])),
                Some(_) => {}
                None => return Err("Expected `)`"),
            }
        }
    }

    fn text(&mut self, start: usize) -> Piece<'a> {
        while let Some(&(pos, ch)) = self.it.peek() {
            match ch {
                '$' => return Piece::Text(&self.pattern[start..pos]),
                _ => {
                    self.it.next();
                }
            }
        }
        Piece::Text(&self.pattern[start..])
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Piece<'a>;

    fn next(&mut self) -> Option<Piece<'a>> {
        match self.it.peek() {
            Some(&(_, '$')) => {
                self.it.next();
                if self.consume('$') {
                    return Some(Piece::Text("$"));
                }
                Some(self.argument())
            }
            Some(&(pos, _)) => Some(self.text(pos)),
            None => None,
        }
    }
}
