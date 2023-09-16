
fn main() {
    println!("Hello, world!");


    assert_eq!(heaviside(5), 1);
    assert_eq!(heaviside(-5), 0);
    assert_eq!(heaviside(0), 0);


    let x = 123;
    let y = 432;
    println!("swapping x: {x} with y: {y}.");
    
    let (new_x, new_y) = swap(x, y);

    println!("now x: {new_x}, y: {new_y}.");

    let x = -213;
    let abs_x = abs(x);
    println!("x: {x}, abs_x: {abs_x}.");

    assert_eq!(signum(0), 1);
    assert_eq!(signum(32), 1);
    assert_eq!(signum(-2), -1);


    zadanie2();
    zadanie3();
    zadanie4();

}

fn zadanie4(){
    let quadrant1 = which_quadrant((1., 1.));
    let quadrant2 = which_quadrant((-3., 2.));
    let quadrant3 = which_quadrant((-1., -1.));
    let quadrant4 = which_quadrant((4., -1.));
    assert_eq!(1, quadrant1);
    assert_eq!(2, quadrant2);
    assert_eq!(3, quadrant3);
    assert_eq!(4, quadrant4);
}

fn which_quadrant(point: (f64, f64)) -> i32{
    if point.0 < 0. && point.1 < 0. {return 3};
    if point.0 < 0. {return 2};
    if point.1 < 0. {return 4};
    return 1;
}

fn zadanie3(){
    println!("zadanie3");

    let point = (2.0, 3.0);
    let a = (0., 0.);
    let b = (10., 0.);
    let c = (0., 10.);
    assert!((calculate_triangle_area(a, b, c) - 50.0) < 0.0005);
    assert!((calculate_triangle_area(a, c, b) - 50.0) < 0.0005);
    assert!((calculate_triangle_area(b, a, c) - 50.0) < 0.0005);
    assert!((calculate_triangle_area(b, c, a) - 50.0) < 0.0005);
    assert!((calculate_triangle_area(c, a, b) - 50.0) < 0.0005);
    assert!((calculate_triangle_area(c, b, a) - 50.0) < 0.0005);

    let is_point_inside = is_point_inside_triangle(a, b, c, point);
    let custom_text = if is_point_inside {""} else {" not"};
    println!("point {point:?} is{custom_text} inside triangle with vertices: {a:?}, {b:?}, {c:?}");


    let point = (7., 5.);
    let a = (0., 0.);
    let b = (10., 0.);
    let c = (0., 10.);

    let is_point_inside = is_point_inside_triangle(a, b, c, point);
    let custom_text = if is_point_inside {""} else {" not"};
    println!("point {point:?} is{custom_text} inside triangle with vertices: {a:?}, {b:?}, {c:?}");
}


// (x, y)
fn is_point_inside_triangle(a: (f64, f64), b: (f64, f64), c: (f64, f64), point: (f64, f64)) -> bool{
    let triangle_area  =  calculate_triangle_area(a, b, c);
    let area1 = calculate_triangle_area(a, b, point);
    let area2 = calculate_triangle_area(a, c, point);
    let area3 = calculate_triangle_area(b, c, point);
    println!("triangle_area: {triangle_area}");
    println!("area1: {area1}");
    println!("area2: {area2}");
    println!("area3: {area3}");
    return (area1 + area2 + area3 - triangle_area).abs() < 0.00005;
}

fn calculate_triangle_area(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64{
    return 0.5 * ((a.0 * (b.1 - c.1) + b.0 * (c.1 - a.1) + c.0 * (a.1 - b.1))).abs();
}

fn zadanie2(){
    println!("zadanie2");

    let a = 3.0;
    let b = 4.0;
    let c = 5.0;

    let is_triangle_possible = check_if_triangle_is_possible(a, b, c);
    let custom_text = if is_triangle_possible {"can"} else {"cannot"};

    println!("from sections of length {a}, {b}, {c} the triangle {custom_text} be constructed");

    let a = 1.0;
    let b = 2.0;
    let c = 5.0;
    let is_triangle_possible = check_if_triangle_is_possible(a, b, c);
    let custom_text = if is_triangle_possible {"can"} else {"cannot"};

    println!("from sections of length {a}, {b}, {c} the triangle {custom_text} be constructed");
}

fn check_if_triangle_is_possible(a: f64, b: f64, c: f64) -> bool{
    return a + b > c && a + c > b && b + c > a;
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
    return if x < 0 {-x} else {x}
}

fn signum(x: i32) -> i32{
    return if x < 0 {-1} else {1}
}