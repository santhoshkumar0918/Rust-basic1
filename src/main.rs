// fn main() {
//     // Define the enum for cardinal directions
//     enum CardinalDirection {
//         NORTH,
//         EAST,
//         SOUTH,
//         WEST,
//     }

//     // Assign the variable `d` to the `EAST` variant of `CardinalDirection`
//     let d = CardinalDirection::EAST;

//     // Check if the direction is EAST
//     if let CardinalDirection::EAST = d {
//         println!("We are going east!");
//     } else {
//         println!("We are not going east but in some other direction!");
//     }
// }





// Refernces and pointers (immutable)

// fn main() {
// struct Config {
//     port: u16,
// }
// let config: Config = Config {
//     port: 8080
// };
// // Create a reference.
// let config_reference: &Config = &config;
// // In some other part of the program, use the reference.
// println!("Using port {}.", config_reference.port);
// }
// mutable
// fn main(){
//     struct Config {
//         port : u16,
//     }

//     let mut config : Config = Config{
//         port : 8000
//     };

//     let config_reference : &mut Config = &mut config;
//     config_reference.port = 4000;
//     println!("using port {}",config_reference.port);
// }

// LET PATTERN 

// fn main() {
//     // Define the Person struct
//     struct Person {
//         name: &'static str,
//         age: u32,
//         likes_games: bool,
//     }

//     // Create an instance of Person
//     let p = Person {
//         name: "santhosh",
//         age: 18,
//         likes_games: true,
//     };

//     // Destructure the Person instance
//     let Person {
//         name,
//         age,
//         likes_games,
//     } = p;

//     // Print the values to verify
//     println!("Name: {}", name);
//     println!("Age: {}", age);
//     println!("Likes Games: {}", likes_games);
// }

// fn main() {
// let value = 0;
// if value > -10 && value < 10 {
//     println!("Single digit!");
// } else {
//     println!("Multiple digits!");
// }
// }

// fn main() {

// let meal = Meal::Hamburger {
//     vegetarian: true,
// };

// if let Meal::Hamburger { vegetarian: true } = meal {
//     println!("I had a vegetarian hamburger!");
// }
// }

// fn main() {
// for n in 0..=5 {
//     match n {
//         1 => println!("It was one!"),
//         2 => println!("It was two!"),
//         // or-pattern
//         3 | 4 => println!("It was a bit more than two!"),
//         6 => println!("vdksv v"),
//         high if high >= 5 => println!("It was a high number!"),
//         // a pattern consisting only of the name `other`
//         other => println!("It was {other}!"),
//     }
// }
// }

// fn main(){
//  for  n in 0..=5{
//     match n {
//         1 => println!("it was  one"),
//         3 => println!("it was a  two"),
//         6 => println!("it was a  error "),
//         5 => println!("it was a  correct and high number"),
//         7 => println!("jbc "),

//         high if high >= 8 => println!("ccjshch "),
//         other => println!("it was {other}")
//     }
//  }
// }


// fn main() {
// enum Meal {
//     FishAndChips { chip_proportion: f64 },
//     Hamburger { vegetarian: bool },
// }

// let meal = Meal::FishAndChips {
//     chip_proportion: 0.6,
// };

// let meal = Meal::Hamburger { vegetarian: true };

// match meal {
//     Meal::FishAndChips { chip_proportion } => {
//         if chip_proportion > 0.5 {
//             println!("I had some fish and plenty of chips!");
//         } else if chip_proportion < 0.5 {
//             println!("I had plenty of fish and some chips!");
//         } else {
//             println!("I had fish and chips!");
//         }
//     }
//     Meal::Hamburger { vegetarian } => {
//         if vegetarian {
//             println!("I had a vegetarian hamburger!");
//         } else {
//             println!("I had a meaty hamburger!");
//         }
//     }
// }
// }


// #![allow(unused)]
// fn main() {
// enum Meal {
//     FishAndChips { chip_proportion: f64 },
//     Hamburger { vegetarian: bool },
// }

// let meal = Meal::Hamburger { vegetarian: true } ;
//   let meal = Meal::FishAndChips { chip_proportion: 0.6 };

// match meal {
//     Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
//         println!("I had some fish and plenty of chips!");
//     }
//     Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
//         println!("I had plenty of fish and some chips!");
//     }
//     Meal::FishAndChips { chip_proportion } => {
//         println!("I had fish and chips!");
//     }
//     Meal::Hamburger { vegetarian: true } => {
//         println!("I had a vegetarian hamburger!");
//     }
//     Meal::Hamburger { vegetarian: false } => {
//         println!("I had a meaty hamburger!");
//     }
// }
// }


// fn main(){
//     enum Meal {
//         FishAndChips {chips_production : f64},
//         Hamburger {vegeterian : bool},
//     }

//     // let mut meal  = Meal::FishAndChips { chips_production: 0.8 };
//     let mut meal = Meal::Hamburger { vegeterian: true };

//    while let Meal::Hamburger { vegeterian } = meal {
//        println!("Going to have fish with chips {:.2}...",vegeterian);

//        if vegeterian > false {
//         meal =  Meal::Hamburger  {  vegeterian : true };

//        }
//        else {
//            meal = Meal::Hamburger { vegeterian: false }
//        }

//    }
   
//        println!("i am done with both fishchips and veg")
// }

// fn main(){
//     let print_cookies = 5;
//     let   qauntity;

//     if print_cookies == 3 {
//         qauntity = " 3 cookies";

//     }else {
//         qauntity = "i didnt have proper cookies";
//     }

//     println!("i had {qauntity} today:")
// }

// fn main(){
//     let mut i = 0;
//     let x = loop{
//         if i > 5 {
//             break i;
//         }
//         i += i*2 + 3;
//     };
// println!("{x}")
// }

// fn main (){
//     struct  Sa (&'static str);
//         impl Sa {
//             fn associated_fn() -> &'static str{
//                 " talent X hardwork"
//             }

//             fn method(self : &Self) -> &'static str {
//                 self.0
//             }
//         }
//     println!("{}",Sa::associated_fn());

//     let instance = Sa("my  value depends on Sa func.");
//     println!("{}",instance.method());
// }

fn main(){
//    let c = |x| {
//     x * 2
// };
// println!("{}",c(3));


let a  = [2,3,4];
let n : i32  = a.iter().map(|x| x * 3 ).sum();
println!("sum of {:?}vvdv{}",a,n);
}














