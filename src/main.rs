fn main() {
    /*
    Tuple structs have unnamed fields.
    Their type is declared but no value names are given

    */  
    
    //tuple struct declaration
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("the first example of a tuple struct values for black color is: {} {} {}", black.0, black.1, black.2);
    println!("the three points declared are: {} {} {}",origin.0, origin.1, origin.2);
   
}


