// move_semantics2-a.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// rustc exercises/move_semantics/move_semantics2-a.rs && ./move_semantics2-a

fn main() {
    let mut vec0 = Vec::new();

    let vec1 = fill_vec(&mut vec0);

    // Uncomment this line and there's an error. The `println!` tries to do an immutable
    // borrow after `fill_vec` does a mutable borrow. Compare with `move_semantics2.rs`.
    /*
        error[E0502]: cannot borrow `vec0` as immutable because it is also borrowed as mutable
      --> exercises/move_semantics/move_semantics2-a.rs:13:57
       |
    10 |     let vec1 = fill_vec(&mut vec0);
       |                         --------- mutable borrow occurs here
    ...
    13 |     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
       |                                                         ^^^^^^^^^^ immutable borrow occurs here
    14 |
    15 |     vec1.push(88);
       |     ------------- mutable borrow later used here

    error[E0502]: cannot borrow `vec0` as immutable because it is also borrowed as mutable
      --> exercises/move_semantics/move_semantics2-a.rs:13:69
       |
    10 |     let vec1 = fill_vec(&mut vec0);
       |                         --------- mutable borrow occurs here
    ...
    13 |     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
       |                                                                     ^^^^ immutable borrow occurs here
    14 |
    15 |     vec1.push(88);
       |     ------------- mutable borrow later used here
       |
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

    error: aborting due to 2 previous errors

    For more information about this error, try `rustc --explain E0502`.
    */
    //println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
