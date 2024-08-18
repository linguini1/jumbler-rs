use std::io::{BufRead, Write};

/// Represents a state of the Jumbler FSM.
#[derive(Debug)]
enum State {
    /// The start state of the FSM
    Start,
    /// The first character of a word
    FirstChar,
    /// The nth character of a word (not first)
    NthChar,
    /// The end of a word which needs to be jumbled
    Jumble,
    /// A non-alphabetic character
    NonAlpha,
}

impl State {
    /// Return the next state in the transition path based on the character input
    fn next(&self, c: char) -> Self {
        match self {
            Self::Start => {
                if c.is_alphabetic() {
                    Self::FirstChar
                } else {
                    Self::NonAlpha
                }
            }
            Self::FirstChar => {
                if c.is_alphabetic() {
                    Self::NthChar
                } else {
                    Self::NonAlpha
                }
            }
            Self::NthChar => {
                if c.is_alphabetic() {
                    Self::NthChar
                } else {
                    Self::Jumble
                }
            }
            Self::Jumble => {
                if c.is_alphabetic() {
                    Self::FirstChar
                } else {
                    Self::NonAlpha
                }
            }
            Self::NonAlpha => {
                if c.is_alphabetic() {
                    Self::FirstChar
                } else {
                    Self::NonAlpha
                }
            }
        }
    }
}

/// Represents the FSM that performs the jumbler logic
pub struct JumblerFSM<'a> {
    /// Current state of the FSM
    state: State,
    /// Source of character stream
    output: &'a mut dyn Write,
    /// Last read character
    lastchar: char,
    /// Buffer of characters to jumble
    chars: Vec<char>,
}

impl<'a> JumblerFSM<'a> {
    /// Initialize a new Jumbler FSM
    pub fn new(output: &'a mut dyn Write) -> Self {
        JumblerFSM {
            state: State::Start,
            output,
            lastchar: '\0',
            chars: Vec::new(),
        }
    }

    /// Move the FSM to its next state based on the last input
    fn next_state(&mut self) {
        self.state = self.state.next(self.lastchar);
    }

    /// Execute state specific logic
    fn execute(&mut self) {
        match self.state {
            State::Start => {}
            State::NonAlpha => {
                self.output.write_all(&[self.lastchar as u8]).unwrap();
            }
            State::FirstChar => {
                self.output.write_all(&[self.lastchar as u8]).unwrap();
            }
            State::NthChar => {
                self.chars.push(self.lastchar);
            }
            State::Jumble => {
                self.jumble();
                for c in self.chars.iter() {
                    let _ = self.output.write_all(&[*c as u8]);
                }
                let _ = self.output.write_all(&[self.lastchar as u8]); // Ending non-alpha
                self.chars.clear(); // Reset buffer
            }
        };
    }

    /// Shuffle all stored characters up to the last item
    fn jumble(&mut self) {
        let n = self.chars.len() - 1;
        if n <= 1 {
            return;
        }
        for i in 0..(n - 1) {
            let j = rand::random::<usize>() % (n - i) + i;
            self.chars.swap(i, j)
        }
    }

    /// Execute the FSM to completion
    pub fn run<S: BufRead>(&mut self, source: S) {
        for byte in source.bytes().flatten() {
            self.execute();
            self.lastchar = byte as char;
            self.next_state();
        }
        self.execute();
        self.lastchar = '\0';
        self.next_state();
        self.execute();
    }
}
