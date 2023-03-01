fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");

    let j_str = "new string variable";
    println!("The j_str val is: {}", j_str);

    {
        let mut x = 12;
        println!("The value of x in inner scope is: {x}");
        //x = 15;
        //println!("The value of x in inner scope again is: {x}");
        x = 1;
        println!("The value of x in inner scope again is: {x}");
        let j_str = j_str.len();
        println!("The j_str len was: {}", j_str);
    }
    println!("The j_str val is again: {}", j_str);

    x = 6;
    println!("The value of x is again: {x}");

    let tup: (i32, f32, u8) = (1024, 17.5, 7);

    let (x, y, z) = tup;

    println!("The values of x, y, z are now: {x}, {y}, {z}");
}
