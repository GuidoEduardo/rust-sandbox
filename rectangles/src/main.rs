#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;

    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rectangle);

    println!(
        "The area of the rectangles is {} square pixels.",
        rectangle.area()
    );
}
