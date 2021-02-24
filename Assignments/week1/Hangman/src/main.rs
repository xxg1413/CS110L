// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    
    println!("random word: {}", secret_word);

    let guess_len = secret_word_chars.len();
    let mut guessed: Vec<char> = vec!['-'; guess_len];

    
    //println!("{:?}",guessed);

    let mut sum_guess =  NUM_INCORRECT_GUESSES;
    

    while sum_guess > 0
    {

        //let mut s: String = guessed.into_iter().collect();

        println!("欢迎来到猜单词游戏");
       // println!("这是你要猜的单词: {}",s);
        println!("你现在还有{}次机会\n", sum_guess);


        print!("请猜一个字母：");

        io::stdout().flush().expect("标准输出错误");


        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读入错误");
        
        println!("你输入的是：{}", guess);

        let guess_vec: Vec<char> = guess.chars().collect();

        println!("你输入的是：{}", guess_vec[0]);

        
        if secret_word_chars.contains(&guess_vec[0]) 
        {
            let index = secret_word_chars.iter().position( |&r|r == guess_vec[0]).unwrap();
            println!("{}",index);

            guessed[index] = guess_vec[0];
            println!("你猜对了");

        }
        else 
        {
            println!("你猜错了");
            sum_guess -= 1;
        }

        println!("你全部猜错了，继续加油！");

    }

}
