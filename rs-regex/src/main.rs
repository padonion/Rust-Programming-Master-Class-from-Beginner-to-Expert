extern crate regex;
use regex::Regex;

fn main() {

    let re = Regex::new(r"[prt]ain").unwrap();
    let text = "Spain, Spain, plain, rain, train, Spain";
    
    println!("the text has a match: {:?}", re.is_match(text));
    println!("the text has a match: {:?}", re.find(text));
    
    for capture in re.captures_iter(text) {
        println!("Found match: {:?}", capture.get(0).unwrap().as_str());
    }
}
