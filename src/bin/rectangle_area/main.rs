mod structs;
use crate::structs::Rectangle;

fn main() {
  let rectangle: Rectangle = Rectangle { width: 30, height: 50 };
  let area: u32 = rectangle.calculate_area();

  let rect2: Rectangle = Rectangle { width: 10, height: 40 };
  let rect3: Rectangle = Rectangle { width: 60, height: 45 };

  let square: Rectangle = Rectangle::create_square(10);

  println!("The ractangle area is: {area}");
  println!("");
  println!("Debug info for the rectangle: {:#?}", rectangle);
  println!("");
  dbg!(&rectangle);
  println!("");
  println!("Is width valid? {}", rectangle.is_width_valid());
  println!("");
  println!("Is height valid? {}", rectangle.is_height_valid());
  println!("");
  println!("can rect hold rect2? {}", rectangle.can_hold_rectangle(&rect2));
  println!("can rect hold rect3? {}", rectangle.can_hold_rectangle(&rect3));
  println!("");
  dbg!("this is a square: {}", &square);

}
