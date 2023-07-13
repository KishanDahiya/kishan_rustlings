// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());
    vec0 = vec1.clone();
    //This is very confusing too? why when i cloned the vec0 in line 15 and comment line 16 it will give below result as length 0?
    //But when i uncomment i get the desired output as in the comments given at the start? (line 4 and 5)

    //i want the solution as per this hint below
    //3. Or, you could make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
    //mutable), modify it directly, then not return anything. This means that `vec0` will change over the
    //course of the function, and makes `vec1` redundant (make sure to change the parameters of the `println!`
    //statements if you go this route)
    //solution as per above
    //fill_vec(&mut vec0);

    println!(
        "{} has length {}, with contents: `{:?}`",
        "vec0",
        vec0.len(),
        vec0
    );

    vec1.push(88);

    println!(
        "{} has length {}, with contents `{:?}`",
        "vec1",
        vec1.len(),
        vec1
    );
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
