use std::io;
fn main() {
    loop {
        let mut guess = String::new();
        println!("what's the num?");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim() == "exit" {
            break
        }
        let num = guess.trim().parse::<i32>()
            .expect("Failed to parse");
        println!("num: {}", num);

        if num == 12 {
            println!("correct");
            break
        } else {
            println!("try again");
        }
    }
}