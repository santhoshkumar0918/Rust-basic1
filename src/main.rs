// // fn main() {
// //     // Define the enum for cardinal directions
// //     enum CardinalDirection {
// //         NORTH,
// //         EAST,
// //         SOUTH,
// //         WEST,
// //     }

// //     // Assign the variable `d` to the `EAST` variant of `CardinalDirection`
// //     let d = CardinalDirection::EAST;

// //     // Check if the direction is EAST
// //     if let CardinalDirection::EAST = d {
// //         println!("We are going east!");
// //     } else {
// //         println!("We are not going east but in some other direction!");
// //     }
// // }





// // Refernces and pointers (immutable)

// // fn main() {
// // struct Config {
// //     port: u16,
// // }
// // let config: Config = Config {
// //     port: 8080
// // };
// // // Create a reference.
// // let config_reference: &Config = &config;
// // // In some other part of the program, use the reference.
// // println!("Using port {}.", config_reference.port);
// // }
// // mutable
// // fn main(){
// //     struct Config {
// //         port : u16,
// //     }

// //     let mut config : Config = Config{
// //         port : 8000
// //     };

// //     let config_reference : &mut Config = &mut config;
// //     config_reference.port = 4000;
// //     println!("using port {}",config_reference.port);
// // }

// // LET PATTERN 

// // fn main() {
// //     // Define the Person struct
// //     struct Person {
// //         name: &'static str,
// //         age: u32,
// //         likes_games: bool,
// //     }

// //     // Create an instance of Person
// //     let p = Person {
// //         name: "santhosh",
// //         age: 18,
// //         likes_games: true,
// //     };

// //     // Destructure the Person instance
// //     let Person {
// //         name,
// //         age,
// //         likes_games,
// //     } = p;

// //     // Print the values to verify
// //     println!("Name: {}", name);
// //     println!("Age: {}", age);
// //     println!("Likes Games: {}", likes_games);
// // }

// // fn main() {
// // let value = 0;
// // if value > -10 && value < 10 {
// //     println!("Single digit!");
// // } else {
// //     println!("Multiple digits!");
// // }
// // }

// // fn main() {

// // let meal = Meal::Hamburger {
// //     vegetarian: true,
// // };

// // if let Meal::Hamburger { vegetarian: true } = meal {
// //     println!("I had a vegetarian hamburger!");
// // }
// // }

// // fn main() {
// // for n in 0..=5 {
// //     match n {
// //         1 => println!("It was one!"),
// //         2 => println!("It was two!"),
// //         // or-pattern
// //         3 | 4 => println!("It was a bit more than two!"),
// //         6 => println!("vdksv v"),
// //         high if high >= 5 => println!("It was a high number!"),
// //         // a pattern consisting only of the name `other`
// //         other => println!("It was {other}!"),
// //     }
// // }
// // }

// // fn main(){
// //  for  n in 0..=5{
// //     match n {
// //         1 => println!("it was  one"),
// //         3 => println!("it was a  two"),
// //         6 => println!("it was a  error "),
// //         5 => println!("it was a  correct and high number"),
// //         7 => println!("jbc "),

// //         high if high >= 8 => println!("ccjshch "),
// //         other => println!("it was {other}")
// //     }
// //  }
// // }


// // fn main() {
// // enum Meal {
// //     FishAndChips { chip_proportion: f64 },
// //     Hamburger { vegetarian: bool },
// // }

// // let meal = Meal::FishAndChips {
// //     chip_proportion: 0.6,
// // };

// // let meal = Meal::Hamburger { vegetarian: true };

// // match meal {
// //     Meal::FishAndChips { chip_proportion } => {
// //         if chip_proportion > 0.5 {
// //             println!("I had some fish and plenty of chips!");
// //         } else if chip_proportion < 0.5 {
// //             println!("I had plenty of fish and some chips!");
// //         } else {
// //             println!("I had fish and chips!");
// //         }
// //     }
// //     Meal::Hamburger { vegetarian } => {
// //         if vegetarian {
// //             println!("I had a vegetarian hamburger!");
// //         } else {
// //             println!("I had a meaty hamburger!");
// //         }
// //     }
// // }
// // }


// // #![allow(unused)]
// // fn main() {
// // enum Meal {
// //     FishAndChips { chip_proportion: f64 },
// //     Hamburger { vegetarian: bool },
// // }

// // let meal = Meal::Hamburger { vegetarian: true } ;
// //   let meal = Meal::FishAndChips { chip_proportion: 0.6 };

// // match meal {
// //     Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
// //         println!("I had some fish and plenty of chips!");
// //     }
// //     Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
// //         println!("I had plenty of fish and some chips!");
// //     }
// //     Meal::FishAndChips { chip_proportion } => {
// //         println!("I had fish and chips!");
// //     }
// //     Meal::Hamburger { vegetarian: true } => {
// //         println!("I had a vegetarian hamburger!");
// //     }
// //     Meal::Hamburger { vegetarian: false } => {
// //         println!("I had a meaty hamburger!");
// //     }
// // }
// // }


// // fn main(){
// //     enum Meal {
// //         FishAndChips {chips_production : f64},
// //         Hamburger {vegeterian : bool},
// //     }

// //     // let mut meal  = Meal::FishAndChips { chips_production: 0.8 };
// //     let mut meal = Meal::Hamburger { vegeterian: true };

// //    while let Meal::Hamburger { vegeterian } = meal {
// //        println!("Going to have fish with chips {:.2}...",vegeterian);

// //        if vegeterian > false {
// //         meal =  Meal::Hamburger  {  vegeterian : true };

// //        }
// //        else {
// //            meal = Meal::Hamburger { vegeterian: false }
// //        }

// //    }
   
// //        println!("i am done with both fishchips and veg")
// // }

// // fn main(){
// //     let print_cookies = 5;
// //     let   qauntity;

// //     if print_cookies == 3 {
// //         qauntity = " 3 cookies";

// //     }else {
// //         qauntity = "i didnt have proper cookies";
// //     }

// //     println!("i had {qauntity} today:")
// // }

// // fn main(){
// //     let mut i = 0;
// //     let x = loop{
// //         if i > 5 {
// //             break i;
// //         }
// //         i += i*2 + 3;
// //     };
// // println!("{x}")
// // }

// // fn main (){
// //     struct  Sa (&'static str);
// //         impl Sa {
// //             fn associated_fn() -> &'static str{
// //                 " talent X hardwork"
// //             }

// //             fn method(self : &Self) -> &'static str {
// //                 self.0
// //             }
// //         }
// //     println!("{}",Sa::associated_fn());

// //     let instance = Sa("my  value depends on Sa func.");
// //     println!("{}",instance.method());
// // }

// // fn main(){
// // //    let c = |x| {
// // //     x * 2
// // // };
// // // println!("{}",c(3));


// // let a  = [2,3,4];
// // let n : i32  = a.iter().map(|x| x * 3 ).sum();
// // println!("sum of {:?}vvdv{}",a,n);
// // }
    

//     // fn main(){
//     //     let is_hello : bool = false;
//     //     let is_mello: bool = false;
        
//     //     if is_hello {
//     //         println!("hello male");
//     //     }
//     //     else {
//     //         println!("hello female")
//     //     }
//     //     if is_hello && is_mello {
//     //         print!("legal")
//     //     }
// //     // }
// //  struct User {
// //     name: String,
// //     age: u64,
// //     email: String,
// //     your_skills_count: u64,
// //     active: bool,
// // }

// // fn main() {
// //     let user1 = User {
// //         name: String::from("ssss"),
// //         age: 18,
// //         email: String::from("santhos@gmail.com"),
// //         your_skills_count: 5,
// //         active: true,
// //     };
    
// //     // Using {} instead of {:?} for string type
// //     println!("User1 username: {}", user1.name);
// //         println!("User1 age: {}", user1.age);
// //     println!("User1 email: {}", user1.email);
// //         println!("User1 skills: {}", user1.your_skills_count);
// //     println!("User1 active: {}", user1.active);

// // }



// // // Recapping Part 1
// // Q. Write a function is_even that takes a number as an input
// // and returns true if it is even


// // fn main(){
// //     println!("{}",is_even(31))
// // }
// // fn is_even(num : i32) -> bool{
// //    if num % 2 == 0 {
// //     return  true;
// //    }else {
// //        return false;
// //    }
// // }


// // Recapping Part 1
// // Q. Write a function fib that finds the fibbonacci of a number
// // it takes as input

// // use std::result;

// // fn main(){
// //     let x : i32 = 1;
// //     print!("{}",x);

// //     let num = 10;
// //     let result = fib(num);
// //     println!("Fibonaacci series of {} is {}",num,result);
// // }
// // fn fib(num  : i32) -> i32{
// //     let mut first = 0;
// //     let mut second = 1;
// //     if num == 0 {
// //         return first;
// //     }
// //     if num == 1{
// //         return second ;
// //     }
// //     for _ in 1..=num - 2 {
// //         let temp = second;
// //         second = second + first;
// //         first  = temp;
// //     }
// //     return  second;
// // }

// // fn get_string_length(s : &str) -> usize{
// // s.chars().count()
// // }

// // fn main(){
// // let my_string = String::from("Hello  fworld");
// // let length = get_string_length(&my_string);
// // println!("length is : {}", length);
// // }



// // struct Square{
// //   width : u32,
// //   height : u32,
// // }

// // impl Square {
// //     fn area(&self) -> u32{
// //      self.width * self.height 

// //      }
// //     fn perimeter(&self ) -> u32{
// //       self.height * self.height
// //     }

// //     }
// // fn main(){
// //     let square = Square{
// //         width : 12,
// //         height : 34,
// //     };
// //     print!("the area of the square {}", square.area());
// //         print!("the area of the square {}", square.perimeter())

// // }



//     // enum Cricket {
//     //     Bat(u32,u32),
//     //     Ball(u32,u32),
//     //     Stump(u32,u32),
//     // }

//     // fn main(){
//     //     let  bat = Cricket::Bat(1,3);
//     //     calculate_area(bat);
//     //     let bowl = Cricket::Ball(2,3);
//     //     calculate_area(bowl);
//     //     let stump = Cricket::Stump(3,3);
//     //     calculate_area(stump)


//     // }

//     // fn calculate_area(value : Cricket) -> u32{
//     //   match value {
//     //     Cricket::Ball(a, b) => a * b,
//     //     Cricket::Bat(a ,b) => a -b ,
//     //     Cricket::Stump(a , c ) => a -    c,
//     //   }
//     // }


    
// // struct Square{
// //   width : u32,
// //   height : u32,
// // }

// // impl Square {
// //     fn area(&self) -> u32{
// //      self.width * self.height 

// //      }
// //     fn perimeter(&self ) -> u32{
// //       self.height * self.height
// //     }

// //     }
// // fn main(){
// //     let square = Square{
// //         width : 12,
// //         height : 34,
// //     };
// //     print!("the area of the square {}", square.area());
// //         print!("the area of the square {}", square.perimeter())

// // }



// use std::io;
// use rand::Rng;
// use std::time::Duration;
// use std::thread;
// use std::collections::HashSet;

// fn draw_stick_man(incorrect: usize, winner: bool) {
//     match incorrect {
//         0 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |        ");
//             println!(" |        ");
//             println!(" |        ");
//             println!("|_");
//         }
//         1 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |     O  ");
//             println!(" |        ");
//             println!(" |        ");
//             println!("|_");
//         }
//         2 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |     O  ");
//             println!(" |     |  ");
//             println!(" |        ");
//             println!("|_");
//         }
//         3 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |     O  ");
//             println!(" |    /|  ");
//             println!(" |        ");
//             println!("|_");
//         }
//         4 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |     O  ");
//             println!(" |    /|\\ ");
//             println!(" |        ");
//             println!("|_");
//         }
//         5 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |     O  ");
//             println!(" |    /|\\ ");
//             println!(" |    /   ");
//             println!("|_");
//         }
//         6 => {
//             println!("  _____   ");
//             println!(" |     |  ");
//             println!(" |     O  ");
//             println!(" |    /|\\ ");
//             println!(" |    / \\ ");
//             println!("|_");
//         }
//         _ => {
//             println!("Invalid value for incorrect.")
//         }
//     }

//     if winner {
//         let duration = Duration::from_secs_f64(0.5);
//         for _ in 0..3 {
//             println!("          ");
//             println!("          ");
//             println!("          ");
//             println!("       O  ");
//             println!("      \\|/ ");
//             println!("/\\");
//             thread::sleep(duration);

//             println!("          ");
//             println!("          ");
//             println!("          ");
//             println!("       O  ");
//             println!("      /|/ ");
//             println!("/\\");
//             thread::sleep(duration);

//             println!("          ");
//             println!("          ");
//             println!("          ");
//             println!("       O  ");
//             println!("      /|\\ ");
//             println!("/\\");
//             thread::sleep(duration);
//         }
//     }
// }

// fn main() {
//     println!();
//     println!();
//     println!();
//     println!();

//     // Randomly select a word
//     let ans_bank = vec!["Nephi", "Lehi", "Leah", "Alma", "Helman", "Mosiah", "Lisa", "Hannah", "Benjamin", "Ehter", "Jarden", "Ammon", "Abinadi", "Zarahmemla", "Enos", "Jarom", "Omin", "Mormon"];
    
//     let rand_int = rand::thread_rng().gen_range(0..ans_bank.len());
//     let code_word = ans_bank[rand_int].to_uppercase();

//     let mut letter_set: HashSet<char> = HashSet::new();
//     for letter in code_word.chars() {
//         letter_set.insert(letter);
//     }

//     let mut correct_guesses_set: HashSet<char> = HashSet::new();
//     let mut incorrect_guesses: Vec<char> = Vec::new();
//     let mut solved = false;
//     let duration = Duration::from_secs(2);

//     println!("Guess the word to save the man!");

//     while !solved && incorrect_guesses.len() < 6 {
//         draw_stick_man(incorrect_guesses.len(), solved);

//         // Print the current state of the word
//         for letter in code_word.chars() {
//             if correct_guesses_set.contains(&letter) {
//                 print!("{letter} ");
//             } else {
//                 print!("_ ");
//             }
//         }
//         println!();
//         println!();

//         // Print previous guesses
//         println!("Guessed Letters: {:?}", incorrect_guesses);
        
//         println!("Guess a letter!");
//         let mut guess_letter = String::new();
//         io::stdin()
//             .read_line(&mut guess_letter)
//             .expect("Failed to read line");
        
//         let guess_letter = guess_letter.trim().to_uppercase();

//         if guess_letter.len() == 1 {
//             let letter = guess_letter.chars().next().unwrap();

//             if !incorrect_guesses.contains(&letter) && !correct_guesses_set.contains(&letter) {
//                 if code_word.contains(letter) {
//                     correct_guesses_set.insert(letter);
//                     println!("Great job! {letter} is in the word!!");
//                 } else {
//                     incorrect_guesses.push(letter);
//                     println!("{letter} is not in the word!");
//                 }

//                 if letter_set == correct_guesses_set {
//                     solved = true;
//                 }
//             } else {
//                 println!("You already guessed that letter!");
//             }
//         } else {
//             println!("Please enter exactly one letter");
//         }

//         thread::sleep(duration);
//     }

//     draw_stick_man(incorrect_guesses.len(), solved);

//     if solved {
//         println!("You Won!!! He survived!! The word was {code_word}!");
//     } else {
//         println!("You lost! The word was {code_word}");
//     }
// }

// use std::collections::btree_map::Values;

// use rand::seq::index;

// enum CustomOption {
//     Some(i32),
//     None
// }
// fn main(){
//     let index = firt_string(String::from("hallo"));
//     match index {
//     CustomOption::Some(value) => println!("found {}",value),
//     CustomOption::None => println!("not found ")

//     }
// }

// fn firt_string(s:String) -> CustomOption{
//     for (index,c) in s.chars().enumerate(){
//         if c == 'a'{
//             return  CustomOption::Some(index as i32);
//         }    

// }
//   return  CustomOption::None; 
// }


// fn main(){
//  let mut vec=  Vec::new();
//  vec.push(1);
//  vec.push(2);
//  vec.push(3);
 
//  let ans  = even_value(&vec);
//   println!("total vec is {:?}",vec);

//  println!("vec is {:?}",ans)


// }

// fn even_value(v :&Vec<i32>) -> Vec<i32>{
//    let mut new_vec = Vec::new();
//    for val in v {
//     if val % 2 != 0{
//       new_vec.push(*val); 

//     }
//    }
//    return  new_vec;
// }

// use rand::seq::index;



fn main(){
    let mut vec = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ans = odd_func(&mut vec);
    println!("hello {:?}",ans)

}

fn odd_func(vi : &mut Vec<i32>) {
    let mut i = 0 ;
    while i < vi.len(){
        if vi[i] % 2!=0 {
            vi.remove(i);
        }else {
            i +=1;
        }
    }
    ;
}







