// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     breadth:u32,
// }
// fn main(){
//     let rect=Rectangle{
//         width:30,
//         breadth:45,
//     };
//     println!("{:?}",rect);
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self) -> u32 {
      self.width * self.height
    }
}
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of Rectangle {}",rect1.area());
}