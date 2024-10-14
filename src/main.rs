use std::io;
// "cargo build" and then ".\target\debug\hello.exe"
// -or-
// "cargo run"
fn main() {
    enter_name();
    //guess_num();
}

fn enter_name() {
    let mut name1 = String::new();
    println!("what's your name?");
    io::stdin().read_line(&mut name1)
        .expect("name input error");
    println!("hello {}", name1);
}
// fn guess_num() {
//     loop {
//         let mut guess = String::new();
//         println!("what's the num?");
//         io::stdin().read_line(&mut guess)
//             .expect("Failed to read line");
//         if guess.trim() == "exit" {
//             break
//         }
//         let num = guess.trim().parse::<i32>()
//             .expect("Failed to parse");
//         println!("num: {}", num);

//         if num == 12 {
//             println!("correct");
//             break
//         } else {
//             println!("try again");
//         }
//     }
// }