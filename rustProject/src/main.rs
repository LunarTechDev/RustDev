
fn main(){
    //println!("Hello World!");

    let number: u32 = 39; // positive. Limit is 2^32
    let negativenumber: i32 = -50; // positive and negative. Limit is 2^31

    let floatnumber: f32 = -3.1415; // Floats can be positive or negative
    let floatnumber1: f64 = 3.1415926; 


    println!("The whole numbers are: {} and {}", number, negativenumber); // Pretty cool. Useful. The {} is the placeholder. Later I define it
    println!("The float numbers are: {floatnumber} and {floatnumber1}"); // Not as cool, but still cool. Still useful


    let _is_lying: bool = false; // true or false
    println!("This is a lie detector. Are you lying?: {}", _is_lying); 

    let e: char = 'e'; // Single character. Can be letter, number or special character (f, 7, *)
    println!("{}", e)


}
