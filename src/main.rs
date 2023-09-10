fn main() {
    println!("Hello, world!");


    assert_eq!(heaviside(5), 1);
    assert_eq!(heaviside(-5), 0);


    let x = 123;
    let y = 432;
    println!("swapping x: {x} with y: {y}.");
    
    let (new_x, new_y) = swap(x, y);

    println!("now x: {new_x}, y: {new_y}.");

}


fn heaviside(x: i32) -> i32{
    if x > 0{
        return 1;
    }
    return 0;
}

fn swap(x: i32, y: i32) -> (i32, i32){
    return (y, x);
}

fn abs(x: i32) -> i32{
    
}