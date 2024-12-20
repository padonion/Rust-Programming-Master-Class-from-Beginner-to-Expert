// repeat pattern

macro_rules! string_concat {

    () => {
        "nothing".to_string()
    };

    ($($some_str: expr), +) => {
        {
            let mut s = "Start: ".to_string();
            $(s.push_str($some_str);)+
            s
        }
    };
}

macro_rules! vec_mac {
    ($($v: expr), *) => {{
        let mut v = vec![];
        $(v.push($v);)*
        v
    }};
}

fn main() {
    println!("{}", string_concat!());
    println!("{}", string_concat!("Hello"));
    println!("{}", string_concat!("Hello", " ", "World"));

    println!("{:?}", vec_mac!(1, 2, 3, 4, 5));
}
