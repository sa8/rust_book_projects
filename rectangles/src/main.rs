#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width *self.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };
    dbg!(&rect1);

    println!(
        "The area of the rectange {:#?} is {} sqaure pixels.",
        rect1,
        rect1.area()
    );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width*rectangle.height
// }