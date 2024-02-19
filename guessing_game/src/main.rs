use std::io;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    // mutはmutableを定義してるらしい

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
