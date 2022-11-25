
//Learning Rust as a hobby...

// use std::io; //Standard input output 



// fn main(){  // main function first to run
//     println!("Welcome");
//     println!("Entr the number");


//     let mut guess = String::new(); //mutable varibale input type string

//     io::stdin().read_line(&mut guess).expect("Not enterd anaything");

//     println!("you guessed {}",guess);

// }

// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// fn main() {

// let random_number = rand::thread_rng().gen_range(1,100);
// println!("The secreat number is {}",random_number);


// loop{
// println!("Enter your guess");

// let mut guess = String::new(); //mutable varibale input type string

// io::stdin().read_line(&mut guess).expect("Not expexted");

// let guess: u32 = match guess.trim().parse() {
// Ok(num) => num,
// Err(_) => continue
// };


// println!("your gues is {}",guess);

//   match guess.cmp(&random_number) {
//     Ordering::Less=>println!("To Less"),
//     Ordering::Greater=>println!("To big"),
//     Ordering::Equal=> {
//         println!("right guessed");
//         break;
//         }
// }
// }

// }

// fn main(){


//     println!("THis is main function");

//     test_function();
// }

// fn test_function(){
//     println!("This is test function");
// }


// Two Seprate function...
// fn main(){


//     println!("THis is main function");

//     test_function(11,11);
//     agina_test(9,9);
// }

// fn test_function(x: i32, y: i32){
//     println!("This is test function x is {}",x);
//     println!("This is test function y is {}",y);
// }
// fn agina_test(z: i32, g: i32){
//     println!("THis is z function {}",z);
//     println!("THis is g function {}",g);
// }
// main
fn main(){
    println!("THis is Rajesh");
    println!("THis is Kutta")
    
}