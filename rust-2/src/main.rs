fn main() {
    // println!("Hello, world!");
    // own();

    // let harsh = moved(xyz);
    // println!("{}",harsh);
    
    // bow();
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str("world");
    update(&mut s1);
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


fn bow () {
    let s1 = String::from("hello harsh");
    let s2 = &s1;  //pass by reference

    println!("{}",s2);
    println!("{}",s1);
}

fn update(xyz : &mut String){
    xyz.push_str("bete");
}
