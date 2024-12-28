/* tuple struct
 * tuple struct is useful when you want your entire tuple
 * have name and different type then other tuples, like
 * Color and Point both have the same field types but
 * they are different type
 */

struct _Color(i32, i32, i32);
struct _Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    rectangle_one();
    rectangle_two();
}

fn rectangle_one() {
    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixles.", area(rect));

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

fn rectangle_two() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}
