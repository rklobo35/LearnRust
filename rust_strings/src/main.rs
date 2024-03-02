fn main() {
    println!("Hello, world!");
    learn_rust_string();
}

fn learn_rust_string() {
    // rust string types
    // string is strong as vector of bytes vec<u8>, heap allocated, growable and not null terminated
    // String - create/ modify string
    // &str - get a string slice, substring 
    // to add a string, remember String::from()
    let s: String = String::from("this needs to be initialized");
    {
        let s1 = &s;
        let i = 32;
        println!("Value of i: {}, s1 : {}", i, s1);
    } 
    
    // s cannot be used here, as it is already moved.
    // so s and s1 are no longer available
    // let mut sliced_string : String = s;

    let mut sliced_string : String = s; // this works as s is only borrowed by s1
    sliced_string.push_str(". And it needs to be extended"); // sliced_string is mutuable so can be modified

    println!("{}", sliced_string);

    // intentional unused string, use underscore (_var); to avoid warning
    let _string : String;

    let for_slicing : String = String::from("Hello World");     
    let mut new_sliced_string : &str = &for_slicing[..]; // get the full string, since overwritten before use, it throws warning
    let size : usize = for_slicing.len();    
    new_sliced_string = &for_slicing[..size]; // cannot be reassigned before making it mutable
    println!("what is my new sliced string {:?}", new_sliced_string);
    new_sliced_string = &for_slicing[6..size]; // can be played around to get any string slice
    println!("what is my new sliced string {:?}", new_sliced_string); 

    let hello_world : String =  String::from("hello world");
    let first_word = first_word(&hello_world);
    let second_word: &str = second_word(&hello_world);
    // hello_world.clear(); // throws error, as immutable borrow occured in line above
    println!("First word: {}; Second word: {}", first_word, second_word); // here the string is empty (:

}


fn first_word (s : &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word (s : &String) -> &str {
    let bytes = s.as_bytes();
    let len = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[i+1..len];
        }
    }
    &s[..]
}
