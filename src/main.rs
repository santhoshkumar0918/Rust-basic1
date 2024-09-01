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

fn main() {
let value = 0;
if value > -10 && value < 10 {
    println!("Single digit!");
} else {
    println!("Multiple digits!");
}
}





