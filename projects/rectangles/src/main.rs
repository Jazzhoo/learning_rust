fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_area = area(&rect1);
    println!("The area of given rectangle is {}", rect_area);
    println!("The rect1 properties are: \n{:#?}", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height

}
