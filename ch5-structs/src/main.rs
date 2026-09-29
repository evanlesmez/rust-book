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

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50 * scale,
    };
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1);
    //datadriven_rect_area();
    method_rect_area(&rect1);
}

fn method_rect_area(rect: &Rectangle) {
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn datadriven_rect_area(rect: &Rectangle) {
    println!("The area of the rectangle is {} square pixels.", area(rect));
}
