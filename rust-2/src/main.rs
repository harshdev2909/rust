fn main() {
    // println!("Hello, world!");
    // own();

    let xyz = String::from("ladle ");
    let harsh = moved(xyz);
    println!("{}",harsh);
}

fn own () {
    let x:i64 = 10;
    let y = x;
    println!("{}", x);

    let name = String::from("im harsh");
    let uname = name.clone();
    println!("{}",name);
}



fn moved (xyz:String) -> String {
    println!("{}",xyz);
    return xyz;
}


//scoping
/*{

    // x =10; 

   }*/