// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // {
    //     //Shadowing and scope concept
    //     //This below is scope concept, applying it to innerblock
    //     let number:u32 = 3; // don't rename this variable
    //     println!("Number plus two is : {}", number + 2);
    // }
    //This below is shadowing conecpt applied to outer block on same variable
    //Once we have reassigned the variable by not initializing but declaring it will erase previous value
    // from its memoery stack(Buffer) & update it with new value
    let number: u32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
