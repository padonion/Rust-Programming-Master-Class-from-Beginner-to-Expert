// capturing types
// macro_rules! input {
//     ($t: ty) => {{
//         let mut n = String::new();
//         std::io::stdin()
//             .read_line(&mut n)
//             .expect("Failed to read line");
//         let n: $t = n.trim().parse().expect("Invalid input");
//         n
//     }};
// }

macro_rules! add_as {
    ($a:expr, $b:expr, $t:ty) => {
        $a as $t + $b as $t
    };
}

macro_rules! some_macro {
    ($var: ident) => {
        $var += 1;
    };
}


macro_rules! create_function {
    ($func_name:ident, $input: ident, $type_input: ty, $type_output: ty) => {
        fn $func_name($input: $type_input) -> $type_output {
            println!("You called {} with input of {} as type {} and output as type {}", 
                stringify!($func_name), stringify!($input), stringify!($type_input), stringify!($type_output));
            $input as $type_output
        }
    };
}

create_function!(f1, x, i32, f64);

fn main() {

    //println!("Enter a floating point number: ");
    //let _x: f64 = input!(f64);

    println!("result of add_as: {:?}", add_as!(1, 2, f64));

    let mut _x = 1;
    some_macro!(_x);
    println!("result of some_macro: {:?}", _x);

    // try function creation
    let _res = f1(10);
}
