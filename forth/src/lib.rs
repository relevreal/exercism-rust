use std::collections::HashMap;
use std::str::Split;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type ValueResult = std::result::Result<Value, Error>;

pub struct Forth {
    stack: Vec<Value>,
    word_indices: HashMap<String, usize>,
    words: Vec<WordDefinition>,
}

#[derive(Clone, Copy, Debug)]
enum BuiltIn {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
}

#[derive(Clone, Copy)]
enum WordType {
    BuiltIn(BuiltIn),
    UserDefined(usize),
    Number(Value),
}

#[derive(Clone)]
struct WordDefinition {
    definition: Vec<WordType>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn is_number(word: &str) -> bool {
    word.bytes().all(|b| b.is_ascii_digit())
}

fn is_word(word: &str) -> bool {
    let not_digit = !word.bytes().all(|b| b.is_ascii_digit());
    not_digit
        && word
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b.is_ascii_punctuation())
}

fn get_builtin(word: &str) -> Option<BuiltIn> {
    println!("get builtin: {}", word);
    match word {
        "+" => Some(BuiltIn::Add),
        "-" => Some(BuiltIn::Sub),
        "*" => Some(BuiltIn::Mul),
        "/" => Some(BuiltIn::Div),
        "DUP" => Some(BuiltIn::Dup),
        "DROP" => Some(BuiltIn::Drop),
        "SWAP" => Some(BuiltIn::Swap),
        "OVER" => Some(BuiltIn::Over),
        _ => None,
    }
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: vec![],
            word_indices: HashMap::new(),
            words: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.trim().to_uppercase();
        let mut words_iter = input.split(' ');
        while let Some(word) = words_iter.next() {
            match word {
                ":" => self.eval_definition(&mut words_iter)?,
                number if is_number(word) => self.eval_number(number)?,
                word if is_word(word) => self.eval_word(word)?,
                _ => return Err(Error::InvalidWord),
            }
        }
        Ok(())
    }

    fn eval_number(&mut self, word: &str) -> Result {
        let value: Value = word.parse().map_err(|_| Error::InvalidWord)?;
        self.stack.push(value);
        Ok(())
    }

    fn eval_definition(&mut self, words_iter: &mut Split<char>) -> Result {
        let word_name = words_iter.next().ok_or(Error::InvalidWord)?.to_string();
        if !is_word(&word_name) {
            return Err(Error::InvalidWord);
        }
        let mut def: Vec<WordType> = vec![];
        let mut def_ended = false;
        for w in words_iter.by_ref() {
            if w == ";" {
                let word_idx = self.words.len();
                self.word_indices.insert(word_name.clone(), word_idx);
                let word_def = WordDefinition { definition: def };
                self.words.push(word_def);
                def_ended = true;
                break;
            } else if is_number(w) {
                let number: Value = w.parse().map_err(|_| Error::InvalidWord)?;
                let word_type = WordType::Number(number);
                def.push(word_type);
            } else {
                let word_idx = self.word_indices.get(w);
                match word_idx {
                    Some(&usize) => {
                        let word_type = WordType::UserDefined(usize);
                        def.push(word_type);
                    }
                    None => {
                        let builtin = get_builtin(w).ok_or(Error::UnknownWord)?;
                        def.push(WordType::BuiltIn(builtin));
                    }
                }
            }
        }
        if !def_ended {
            return Err(Error::InvalidWord);
        }
        Ok(())
    }

    fn eval_word(&mut self, word: &str) -> Result {
        let word_idx = self.word_indices.get(word);
        match word_idx {
            Some(&word_idx) => {
                let word_def = self.words[word_idx].clone();
                self.eval_user_defined_fn(&word_def)?;
            }
            None => {
                let builtin = get_builtin(word).ok_or(Error::UnknownWord)?;
                self.eval_builtin(&builtin)?;
            }
        }
        Ok(())
    }

    fn eval_user_defined_fn(&mut self, word_def: &WordDefinition) -> Result {
        for wf in word_def.definition.iter() {
            match wf {
                WordType::BuiltIn(builtin) => self.eval_builtin(builtin)?,
                WordType::UserDefined(word_idx) => {
                    let next_word_def = self.words[*word_idx].clone();
                    self.eval_user_defined_fn(&next_word_def)?;
                }
                WordType::Number(number) => {
                    self.stack.push(*number);
                }
            }
        }
        Ok(())
    }

    fn eval_builtin(&mut self, builtin: &BuiltIn) -> Result {
        match builtin {
            BuiltIn::Add => self.add()?,
            BuiltIn::Sub => self.sub()?,
            BuiltIn::Mul => self.mul()?,
            BuiltIn::Div => self.div()?,
            BuiltIn::Dup => self.dup()?,
            BuiltIn::Drop => self.drop()?,
            BuiltIn::Swap => self.swap()?,
            BuiltIn::Over => self.over()?,
        }
        Ok(())
    }

    fn pop(&mut self) -> ValueResult {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn add(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(a + b);
        Ok(())
    }

    fn sub(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(a - b);
        Ok(())
    }

    fn mul(&mut self) -> Result {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(a * b);
        Ok(())
    }

    fn div(&mut self) -> Result {
        let b = self.pop()?;
        if b == 0 {
            return Err(Error::DivisionByZero);
        }
        let a = self.pop()?;
        self.stack.push(a / b);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        let last = *self.stack.last().ok_or(Error::StackUnderflow)?;
        self.stack.push(last);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let a = self.pop()?;
        let b = self.pop()?;
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }

    fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let b = self.stack[self.stack.len() - 2];
        self.stack.push(b);
        Ok(())
    }
}
