use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

const MAX_QUESTION_COUNT: i32 = 3;
const OPERATOR: &[char] = &['+', '-'];

fn main() {
  let mut num_of_correct = 0;

  while num_of_correct < MAX_QUESTION_COUNT {
    let operator = choose_random(&OPERATOR);
    let num1 = rand::thread_rng().gen_range(1..=100);
    let num2 = rand::thread_rng().gen_range(1..=100);

    let question = format!("{} {} {} = ?", num1, operator, num2);
    println!("第{}問 {}", num_of_correct + 1, question);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().ok();

    if input.is_none() {
      println!("数値を入力してください");
      continue;
    }

    let input = input.unwrap();

    let answer = match operator {
      '+' => num1 + num2,
      '-' => num1 - num2,
      _ => unreachable!(),
    };

    if input == answer {
      println!("正解！");
      num_of_correct += 1;

      if num_of_correct == MAX_QUESTION_COUNT {
        println!("全問正解！おめでとう！");
      }
    } else {
      println!("はずれ！正解は{}です", answer);
    }
  }
}

fn choose_random<T: Copy>(items: &[T]) -> T {
  *items.choose(&mut rand::thread_rng()).unwrap()
}
