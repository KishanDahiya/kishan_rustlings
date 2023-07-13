// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// This is almost like the naming we can use when printing

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; /* your pattern here */

    println!("{} is {} years old.", name, age);
}
