// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

// The tuple without any values has a special name, unit.
// This value and its corresponding type are both written () and represent an empty value or an empty return type.
// Expressions implicitly return the unit value if they don’t return any other value.

//Confused about above a little

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second, "This is not the 2nd number in the tuple!")
}
