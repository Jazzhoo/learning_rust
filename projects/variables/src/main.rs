fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");

    {
        let x = 12;
        println!("The value of x in inner scope is: {x}");
        //x = 15;
        //println!("The value of x in inner scope again is: {x}");
    }

    x = 6;
    println!("The value of x is again: {x}");
}
