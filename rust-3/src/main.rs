struct User {
    name : String ,
    active : bool ,
    email : String
}

struct Rect {
    width : u32,
    height : u32
}

enum Direction {
    north ,
    south , 
    east ,
    west
}

enum Shape {
    circle(f64),
    rectenagle(f64)
}

impl Rect {
    fn area(&self)-> u32{
        return self.width * self.height;
    }
    fn peri(&self)->u32{
        return 2*self.width * self.height;
    }
}

fn main() {
    use std::fs;
    let user = User{
        name : "harsh".to_string(),
        active : true ,
        email : "harshsharmaa990@gmail.com".to_string()
    };

    let rect = Rect{
        width : 30,
        height : 40 
    };

    let circle = Shape::circle(2.5);

    let my_direction = Direction::north;
    // println!("{}",my_direction);
    // println!("the area is {}", rect.area());
    // println!("the area is {}", rect.peri());
    // println!("username is {}", user.name);

    let result = calculate(circle);
    println!("{}",result);

    let res = fs::read_to_string("example.txt");
    match res{
     Ok(content) =>{
        println!("{}",content);
     }
     Err(err) =>{
        println!("{}",err);
     }
    }

}


fn calculate(shape : Shape) ->f64 {
    let ans = match shape {
        Shape::circle(radius) =>3.14*radius*radius ,
        Shape::rectenagle(area) => area*area
    };
    return ans;
}