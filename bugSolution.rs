fn main() {
    let mut x = 5;
    { // Using a block to limit the scope of the mutable borrow
        let y = &mut x; 
        *y = 6; 
    }
    
    { // Another block
        let z = &mut x;  
        *z = 7; 
    }
    println!("x = {}", x); 
}
// Alternative solution: Pass ownership
fn modify(mut x: i32) -> i32 {
    x += 10; 
    x
}

fn main() {
    let mut x = 5; 
    x = modify(x); 
    println!("x = {}", x);
}