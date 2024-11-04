use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;
use std::io::stdin;
use std::result;

fn main() {
  let mut memory = Memory::new();
  let mut prev_result: f64 = 0.0;

  println!("入力してください 例: 1 + 2");

  for line in stdin().lines() {
    let line = line.unwrap();

    // 空行が入力されたら終了
    if line.is_empty() {
      break;
    }

    let tokens = Token::split(&line);

    match &tokens[0] {
      Token::MemoryPlus(memory_name) => {
        // メモリへの加算
        let memory_name = memory_name.to_string();
        let result = memory.add(memory_name, prev_result);
        print_output(value);
      }
      // メモリへの減算
      Token::MemoryMinus(memory_name) => {
        let memory_name = memory_name.to_string();
        let result = memory.minus(memory_name, prev_result);
        print_output(value);
      }
      _ => {
        let left = eval_token(&tokens[0], &memory);
        let right = eval_token(&tokens[2], &memory);
        let result = eval_expression(left, right, &tokens[1]);

        print_output(result);
        prev_result = result;
      }
    }
  }
}

fn print_output(value: f64) {
  println!(" => {}", value);
}

struct Memory {
  slots: HahsMap<(String, f64)>,
}

impl Memory {
  fn new() -> Self {
    Self { slots: HashMap::new() }
  }
  fn add(&mut self, slot_name: String, prev_result: f64) -> f64 {
    match self.slots.entry(slot_name) {
      Entry::Occupied(mut entry) => {
        *entry.get_mut() += prev_result;
        *entry.get()
      }
      Entry::Vacant(entry) => {
        entry.insert(prev_result);
        prev_result
      }
    }
  }
  fn get(&self, slot_name: &str) -> f64 {
    self.slots.get(slot_name).copied().unwrap_or(0.0)
  }
}

#[derive(Debug, PartialEq)]
enum Token {
  Number(f64),
  MemoryRef(String),
  MemoryPlus(String),
  MemoryMinus(String),
  Plus,
  Minus,
  Asterisk,
  Slash,
}

impl Token {
  fn parse(value: &str) -> Self {
    match value {
      "+" => Self::Plus,
      "-" => Self::Minus,
      "*" => Self::Asterisk,
      "/" => Self::Slash,
      _ if value.starts_with("mem") => {
        let mut memory_name = value[3..].to_string();
        if value.ends_with('+') {
          memory_name.pop();
          Self::MemoryPlus(memory_name)
        } else if value.ends_with('-') {
          memory_name.pop();
          Self::MemoryMinus(memory_name)
        } else {
          Self::MemoryRef(memory_name)
        }
      }
      _ => Self::Number(value.parse().unwrap()),
    }
  }
  fn split(text: &str) -> Vec<Self> {
    text.split(char::is_whitespace).map(Self::parse).collect()
  }
}

fn eval_token(token: &Token, memory: &Memory) -> f64 {
  match token {
    Token::Number(value) => {
      *value
    }
    Token::MemoryRef(memory_name) => {
      memory.get(memory_name)
    }
    _ => unreachable!(),
  }
}

fn eval_expression(left: f64, right: f64, operator: &Token) -> f64 {
  match operator {
    Token::Plus => left + right,
    Token::Minus => left - right,
    Token::Asterisk => left * right,
    Token::Slash => left / right,
    _ => unreachable!(),
  }
}
