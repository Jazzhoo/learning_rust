fn main() {

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg! (30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width:  25,
        height: 40,
    };
    let rect3 = Rectangle::square(3);

    let rect_area = rect1.area();
    println!("The area of given rectangle is {}", rect_area);
    println!("The rect1 properties are: \n{:#?}", rect1);
    println!("The rect1 width is non zero: {}", rect1.width());
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("The rec3 sizes are w:{} h:{}", rect3.width, rect3.height);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

