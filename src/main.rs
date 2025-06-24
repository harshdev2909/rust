fn main() {
    let x:i32 = 5;
    let y:f64 = 3.14;
    println!("x is: {}", x);
    println!("y is: {}", y);


    // Boolean variable
    let is_male: bool = true;

    if is_male {
        println!("The person is male.");
    } else {       
        println!("not a male.");
    }

    // Character variable
    let greeeting = String::from("Hello, world!");
    println!("{}", greeeting);

    // Accessing a character from the string
    // Note: Strings in Rust are UTF-8 encoded, so we need to handle characters
    // as Unicode scalar values.
    // Here we use `chars()` to get an iterator over the characters in the string.
    // We can then use `nth(0)` to get the first character.
    // If the string is empty, `nth(0)` will return `None`.
    // We can use pattern matching to handle the `Option` returned by `nth(0
    let char1 = greeeting.chars().nth(0);
    match char1 {
        Some(c) => print!("{}", c),
        None => println!("No character found!"),
    }

    // Looping through numbers
    // Here we use a for loop to iterate from 0 to 9 and print each number.
    // The `..` operator creates a range from 0 to 9 (inclusive).
    // The `print!` macro is used to print each number without a newline.
    // If you want to print each number on a new line, you can use `println
    // loop_s();

    let sentence = String::from("Hello, Rust!");
    let first_word = find_first_word(sentence);
    println!("The first word is: {}", first_word);
}



pub fn loop_s() {
    for i in 0..10{
        print!("{}", i);
    }
}

fn find_first_word(sentence: String) -> String {
    let mut ans = String::from("");

    for c in sentence.chars(){
        ans.push_str(c.to_string().as_str());
        if c == ' ' {
            break;
        }
    };
    return ans;

}