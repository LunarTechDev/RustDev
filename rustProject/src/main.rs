
fn main(){
    //println!("Hello World!");


    // Arrays

    let numbers: [i16; 4] = [1,2,3,4]; // We need to pass in the type of the elements and the length of the Array. Arrays can only have 1 datatype for all elements

    println!("The Array consists of: {:?}", numbers); // There is these 2 formats that is the debuggable and display format. 
    // Some things can be printed in the display format and most can be printed in the debuggable format

    let array_string: [&str; 3] = ["Dog","Cat","Parrot"]; // &str is referencing a string. Not actually owning it 

    println!("This is an Array of the string datatype: {:?}", array_string);
    println!("This is the first string of the array: {}", array_string[0]); // Returns first element of the array

    
    // Tuples

    let water_properties = ("Transparent", "Cool", true, [0,1,2,3,4,5]); // You can store whatever datatype you want for each element
    // Remember to use () instead of []

    println!("{:?}", water_properties);

    let magma_properties: (&str, String, bool) = ("Orange", "Hot".to_string(), false);
    println!("{magma_properties:?}");

    let test_tuple: (&str, &str, &str) = ("Apple", "Apple", "Apple");
    println!("{test_tuple:?}");


    // Slices

    let a: String = String::from("Hello, World!");
    let b: &str = &a[0..5]; // References to the first 5 letters of a's string

    println!("This is owned by a: {a}");
    println!("This is b, a reference to a: {b}");



    // String [growable, mutable, owned string type]

    let mut c: String = String::from("Hello, "); //Mutable, so it can be changed
    c.push_str("World!"); // Added "World!" to c

    println!("c prints: {c}"); 


}
