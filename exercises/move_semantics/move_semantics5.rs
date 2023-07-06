// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    //So x is 100 and mutable that means can be updated
    let mut x = 100;
    //y is just a mutable reference of x means y is 100 which is exactly x and x will change when y changes??
    let y = &mut x;
    //derefernces y and its not incresed by 100 so 200? is x 200
    *y += 100;
    //z is just a mutable reference of x means z is 200 which is exactly x and x will change when z changes??
    let z = &mut x;
    //derefernces z and its now incresed by 1000 so 1200? is x 1200
    *z += 1000;
    assert_eq!(x, 1200);

    //I have like 5 questions in this exercise now \(0_0)/
}
