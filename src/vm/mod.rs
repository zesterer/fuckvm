use std::io::{
    stdin,
    stdout,
    Read,
    Write,
};
use crate::Error;

#[derive(Debug)]
pub enum VmError {
    Unmatched(char),
}

pub struct Vm {
    tape: [u8; 10000],
}

impl Vm {
    pub fn new() -> Self {
        Self {
            tape: [0; 10000],
        }
    }

    pub fn get(&self, offset: usize) -> u8 {
        self.tape.get(offset).cloned().unwrap_or(0)
    }

    pub fn set(&mut self, offset: usize, val: u8) {
        self.tape.get_mut(offset).map(|b| *b = val);
    }

    pub fn incr(&mut self, offset: usize, incr: u8) {
        self.tape.get_mut(offset).map(|b| *b = b.wrapping_add(incr));
    }

    pub fn decr(&mut self, offset: usize, decr: u8) {
        self.tape.get_mut(offset).map(|b| *b = b.wrapping_sub(decr));
    }

    pub fn exec(&mut self, code: &str) -> Result<(), Error> {
        let code = code.bytes().collect::<Vec<_>>();

        let mut tape_ptr = 0;
        let mut code_ptr = 0;

        while let Some(i) = code.get(code_ptr) {
            match i {
                b'.' => {
                    stdout()
                        .lock()
                        .write(&[*self.tape
                            .get(tape_ptr)
                            .unwrap_or(&0)
                        ])
                        .unwrap();
                },
                b',' => {
                    self.tape
                        .get_mut(tape_ptr)
                        .map(|b| *b = stdin()
                            .lock()
                            .bytes()
                            .next()
                            .unwrap_or(Ok(0))
                            .unwrap_or(0)
                        );
                },
                b'<' => tape_ptr = tape_ptr.saturating_sub(1),
                b'>' => tape_ptr = tape_ptr.saturating_add(1),
                b'+' => self.incr(tape_ptr, 1),
                b'-' => self.decr(tape_ptr, 1),
                b'[' if self.get(tape_ptr) == 0 => {
                    let mut balance = 1;
                    while balance != 0 {
                        code_ptr += 1;
                        match code.get(code_ptr) {
                            Some(b'[') => balance += 1,
                            Some(b']') => balance -= 1,
                            Some(_) => {},
                            None => return Err(Error::VmError(VmError::Unmatched('['))),
                        }
                    }
                },
                b']' if self.get(tape_ptr) != 0 => {
                    let mut balance = 1;
                    while balance != 0 {
                        code_ptr -= 1;
                        match code.get(code_ptr) {
                            Some(b'[') => balance -= 1,
                            Some(b']') => balance += 1,
                            Some(_) => {},
                            None => return Err(Error::VmError(VmError::Unmatched(']'))),
                        }
                    }
                },
                b':' => {
                    self.tape[0..20].iter().for_each(|b| print!("{}, ", b));
                    print!("\n");
                },
                _ => {},
            }
            code_ptr += 1;
        }

        Ok(())
    }
}
