/*
macro_rules! say_hello {
    (matching pattern) => { rule };
 */

macro_rules! our_macro {
    () => {
        1 + 1
    };

    (I got something for u @_@) => {
        println!("You got me here!");
    };

    ($e1:expr, $e2:expr) => {
        $e1 + $e2
    };

    ($a: expr, $b: expr, $c: expr) => {
        $a * ($b + $c)
    };
}

fn main() {
    println!("Value of macro is {}", our_macro!());
    our_macro!(I got something for u @_@);

    println!("Value of macro is {}", our_macro!(1, 2));
    println!("Value of macro is {}", our_macro!(1, 2, 3));

    our_macro!();
    our_macro!{};
    our_macro![];
}
