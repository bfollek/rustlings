// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// rustc exercises/move_semantics/move_semantics2-b.rs && ./move_semantics2-b

/*
3. Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
   mutable), modify it directly, then not return anything. Then you can get rid
   of `vec1` entirely -- note that this will change what gets printed by the
   first `println!`"""
*/

// I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new();
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
