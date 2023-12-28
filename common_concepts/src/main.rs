//
// // 3.1 - Variables and Mutability
//
// // fn main() {
//    // let mut spaces = "   ";
//    // spaces = spaces.len();
//    // println!("{space}");
// // }
//
//
//
// 3.2 - Data Types
//
    // Scaler Types - int, float, bool, and char
        // Integer
            // u/i8, u/i16, u/i32/, u/i64, u/i128, and u/isize (depends of architecture)
            // let guess: u32 = "42".parse().expect("Not a number"); k
        // Floating point number 
            // f32 (single precision), and f64 (double precision);
            // fn main () {
                // let x = 2.0; // f64
                // let y: f32 = 3.0; // f32
            // }
        // Boolean
            // True, and False
            // fn main() {
                // let t = true;
                // let f: bool = false; // with explicit type annotation
            // }
        // Character 
            // Most primitive alphabetic type 
            // fn main() {
                // let c = 'z';
                // let z: char = 'Z'; // with explicit type annotation
                // let heart_eyed_cat = '😻';
            // }
    // Compound Types - tuples and arrays
        // Tuples 
            // fn main() {
                // let tup: (i32, f64, u8) = (500, 6.4, 3);
            // }
        // Arrays
            // fn main() 
                // let a = [q, r, s, t];
                // let b: [i32; 5] = [1, 3, 4, 6, 9]; // Explicitly annotating type and length
                // let c = [3; 5] // Array contains 3 five times
            // }
           
// 3.3 - Functions 

 // fn main() {
 //     println!("Hello, world!");
 // 
 //     another_function();
 // }
 // 
 // fn another_function() {
 //     println!("Another function.");
 // }
//
// // 3.3.1 Function Parameters 
//
//     fn main () {
//     another_function(5); // here 5 is the parameter 
// }
//
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }
//
// // eg. 
//         fn main() {
//     print_labeled_measurment(5, 'h');
// }
//
//         fn print_labeled_measurment(value: i32, unit_label: char) {
//     println!("The measurment is: {value}{unit_label}");
// }
//
//
// // 3.3.2 Statements and Expressions 

   //
// eg.
//         fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {y}");
// }
//
// // in the example above, "x + 1" is an expression, and it does not end in a semicolon. If it did,
// that would make it a semicolon, and we do not want a stetement because a statement does not
// return a value.
//
//
// 3.3.3 Functions with Return values
//
//
//         fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//     println!("The value of x is: {x}");
// }
//
// fn main() {
//     let x = plus_one(5);
//
//     println!("The value of x is: {x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
//

//
//
//
//
// Anddddd. this is it for chapter 3 of the The Rust Book
