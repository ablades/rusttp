#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl is like a class
impl Rectangle {

    // &mut self would allow us to take ownership and change values of the rect
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect:&Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width:30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    // Associated fn call ---  let sq = Rectangle::square(3);

    // {:?} - debug pretty print struct must implement #[derive(Debug)]
    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

