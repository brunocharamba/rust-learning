mod structs;
use crate::structs::Rectangle;

fn main() {
  let rectangle: Rectangle = Rectangle { width: 30, height: 40 };
  let area: u32 = rectangle.calculate_area();

  println!("The ractangle area is: {area}");
  println!("");
  println!("Debug info for the rectangle: {:#?}", rectangle);
  println!("");
  dbg!(&rectangle);
  println!("");
  println!("Is width valid? {}", rectangle.is_width_valid());
  println!("");
  println!("Is height valid? {}", rectangle.is_height_valid())

}
