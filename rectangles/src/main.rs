fn main() {
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
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1),
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
    println!("The area of rect2 is {}", rect2.area());

    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));

    // associated functions
    impl Rectangle {
        fn square(size: u32) -> Self {
            // `Self` is just an alias for `Rectangle` in `impl` scope
            Self {
                width: size,
                height: size,
            }
        }
    }

    let square = Rectangle::square(10);
    println!("Square is {:?}", square);
}
