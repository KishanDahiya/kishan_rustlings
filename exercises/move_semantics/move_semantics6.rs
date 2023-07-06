// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I didnt quite understand this, i keep forgetting about the ownership and borrow stuff
// My memory is short :) like really short so i keep forgetting stuff i already read
fn main() {
    let data = "Rust is great!".to_string();

    //so if i am not wrong below is just borrowing and not taking ownership which is a reference(static reference)
    get_char(&data);
    // println!("{}",data);
    
    //below is taking the ownership since we sent whole String as parameter/argument
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
