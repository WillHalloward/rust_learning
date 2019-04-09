#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width * self.height > other.width * other.height
    }
    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
}

fn main(){
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    let square = Rectangle::square(40);

    println!("Can rect 1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect 1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {}, or {} square pixels.",
        area(&rect1),
        rect1.area()
    )
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}