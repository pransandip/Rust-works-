// STRUCTS: your own type, your own data structure
// struct = something AND something AND something
// enum = something OR something OR something -> making one choice
struct FileDirectory; // Unit Struct

#[derive(Debug)]
struct Color(u8, u8, u8); // Tuple Struct

#[derive(Debug)]
struct SizeAndColor {
    size: u32,
    color: Color,
} // Named Struct (It's because variable has name)

fn main() {
    let _my_directory = FileDirectory;
    let my_colours = Color(50, 60, 0);

    let size_and_color = SizeAndColor {
        size: 150,
        color: my_colours,
    };

    println!(
        "my_colours: ({},{},{}), size: {}",
        size_and_color.color.0, size_and_color.color.1, size_and_color.color.2, size_and_color.size
    );

    println!("{:?}", size_and_color);
}
