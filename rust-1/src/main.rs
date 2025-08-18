fn main() {
    println!("Hello, world!");
    // new();
    // bl();
    // st();
    lp();

    let xyz = String::from("ram ram ladle");
    let get_first = first(xyz);
    println!("the first word is {}" , get_first);
}

fn new() {
    let x:i32 = 20;    //for const the variable should be in upper case.   for positive we use i and for -ve we use u32
     println!("the value of x = {}", x);

    let y:f32 = 100.1; //for float
    println!("{}",y);
}

fn bl() {

    let let_male :bool = false;

    if let_male {
        println!(" male");
    }
    else {
        println!("not male");
    }
}

fn st() {
    let name:&str = "harsh";
    let greeting = String::from("harsh");

    let char = greeting.chars().nth(0);

   println!("{}",char.unwrap());
}

fn lp() {
    for i in 0..10{
        println!("harsh");
    }
}


fn first(sentence : String ) ->String{
    let mut ans = String::from("");

    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}