use std::io;

fn main(){
    println!("guest number!");
    println!("please input number");
    
    let mut gues = String::new();
    
    io::stdin().read_line(&mut gues).expect("Failed to read line");

    println!("You guessed: {}", gues)
}

fn showCutestNumber(){
    let mut number = 30000;
   println!("number is {}", nubmer)
}