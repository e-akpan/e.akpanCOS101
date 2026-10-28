#[derive(Debug)]
struct Rectangle {
    Width: u32,
    height:u32
}
impl Rectangle {
    fn area(&self) ->u32 {
        // add code here
        self.Width * self.height
    }
}

fn main() {
    // Instantiate the structure
    let small = Rectangle {
        Width:10,
        height:20
    };
    //print the rectangle's area
    println!("Width is {} \n Height is {} \n Area of rectangle is {}", small.Width, small.height, small.area());
}
