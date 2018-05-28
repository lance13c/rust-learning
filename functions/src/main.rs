fn main() {
    another_function(five());  
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    if false {
      return  5
    } else {
      return 6
    }
}