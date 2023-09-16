use num_traits::pow;
use std::collections::LinkedList;


fn main() {
    println!("Hello, world!");


    zadanie1();
    zadanie2();
    zadanie3();
    zadanie4();
    zadanie5();
    zadanie6();
    zadanie7();
    zadanie8(); 
    println!();
}

fn zadanie8(){
    println!("zadanie8");
    let x = 15;
    println!("x: {x}");
    println!("x: {:b}", x);
    println!("x: {:x}", x);
    println!("x: {:X}", x);
    println!("x: {:o}", x);
}

fn zadanie7(){
    println!("zadanie7");

    let pesels = ["90090515836", "92071314764", "81100216357", "80072909146", "90080517455", "90060804786"];
    let sample_pesel = "55030101193";

    validate_pesel(sample_pesel);
    for pesel in pesels{
        let (is_valid, error) = validate_pesel(pesel);
        let is_valid_text = if is_valid {""} else {" not"};
        println!("pesel: {pesel} is{is_valid_text} valid. {error}");
    }
}

fn validate_pesel(pesel: &str) -> (bool, String){
    if pesel.len() != 11{
        return (false, "Pesel length is not 11.".to_string());
    }
    if !has_only_digits(pesel){
        return (false, "Pesel should only contain digits".to_string());
    }

    fn create_error_message(calculated_digit: i32, pesel_digit: i32) -> String{
        return format!("Checksums don't match, calculated checksum: {calculated_digit}, pesel checksum: {pesel_digit}");
    }


    let m = calculate_checksum_digit_m(pesel);

    let pesel_digit = pesel.chars().last().unwrap().to_string().parse::<i32>().unwrap();
    if m == 0 {
        let are_matching = pesel_digit == 0;
        let error_message = if are_matching {"".to_string()} else {create_error_message(m, pesel_digit)};
        return (are_matching, error_message);
    }
    let calculated_digit = 10 - m; 
    let are_matching = calculated_digit == pesel_digit;
    let error_message = if are_matching {"".to_string()} else {create_error_message(calculated_digit, pesel_digit)}; 
    return (are_matching, error_message);
}

fn has_only_digits(pesel: &str) -> bool{
    return pesel.chars().all(|x| char::is_numeric(x));
}

fn calculate_checksum_digit_m(pesel: &str) -> i32{
    let weights = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];
    let digits = pesel
        .chars()
        .take(10)
        .map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let weights_and_digits = weights.into_iter().zip(digits);

    let sum = weights_and_digits.fold(0, |acc, (weight, digit)| acc + weight * digit);
    return sum % 10;
}


fn zadanie6(){
    println!("zadanie6");
    let sample_text = "Ala ma kota".to_string();
    let tokens = tokenize(sample_text);

    println!("");
    for token in tokens{
        println!("token: {token}");
    }
}

fn tokenize(text: String) -> LinkedList<String>{
    let mut token_list: LinkedList<String> = LinkedList::from([]);

    let char_iterator = text.chars();

    let mut current_token = "".to_string();
    for c in char_iterator {
        print!("{}", c);
        if char::is_whitespace(c){
            if current_token.is_empty(){
                continue;
            }
            token_list.push_back(current_token.clone());
            current_token = "".to_string();
        }
        else{
            current_token.push(c);
        }
    }
    if !current_token.is_empty() {
        token_list.push_back(current_token)
    }

    return token_list;
}


// fn tokenize_recursion_start(test: String){

// }

// fn tokenize_recursion(text: String,) -> LinkedList<String>{
//     let token_list: LinkedList<String> = LinkedList::from([]);

//     return token_list;
// }


fn zadanie5(){
    println!("zadanie5");

    let x = 2;
    assert_eq!(pow(x,4), x << 3);

    assert_eq!(x, pow(x, 4) >> 3);

}

// returns 1, 2, 3, 4
fn zadanie4(){

    println!("zadanie4");

    assert_eq!(point_quarter((1., 1.)), 1);
    assert_eq!(point_quarter((-1., 1.)), 2);
    assert_eq!(point_quarter((-1., -1.)), 3);
    assert_eq!(point_quarter((1., -1.)), 4);

}

fn point_quarter(point: (f64, f64)) -> i32{

    if point.0 < 0. {
        if point.1 < 0. { return 3}
        return 2
    }
    if point.1 < 0. { return 4;}
    return 1;
}

fn zadanie3(){
    println!("zadanie3");

    let point: (f64, f64) = (6., 4.);

    let a = (0., 10.);
    let b = (10., 0.);
    let c = (0., 0.);

    println!("triangle spanned between points: A: {:?}, B: {:?}, C: {:?}", a, b, c);
    let area = triangle_area_points(a, b, c);
    
    println!("area: {area}");
    let custom_text = if is_point_inside_triangle(a, b, c, point) {""} else {" not"};
    println!("the point {:?} is{custom_text} inside triangle.", point);

    let point: (f64, f64) = (6., 4.);
    let custom_text = if is_point_inside_triangle(a, b, c, point) {""} else {" not"};
    println!("the point {:?} is{custom_text} inside triangle.", point);
}

fn is_point_inside_triangle(a: (f64, f64), b: (f64, f64), c: (f64, f64), point: (f64, f64)) -> bool{
    let area1 = triangle_area_points(a, b, point);
    let area2 = triangle_area_points(a, c, point);
    let area3 = triangle_area_points(b, c, point);

    let triangle_area = triangle_area_points(a, b, c);

    return ((area1 + area2 + area3) - triangle_area).abs() < 0.0005;

}

// expects coordinates
fn triangle_area_points(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64{
    return 0.5 * (a.0 * (b.1 - c.1) + b.0 * (c.1 - a.1) + c.0 * (a.1 - b.1)).abs();
}

fn zadanie2(){
    println!("zadanie2");
    let a = 3.;
    let b = 4.;
    let c = 5.;
    assert!(can_create_triangle(a, b, c));
    assert!(can_create_triangle(a, c, b));
    assert!(can_create_triangle(b, a, c));
    assert!(can_create_triangle(b, c, a));
    assert!(can_create_triangle(c, a, b));
    assert!(can_create_triangle(c, b, a));

    let area1 = triangle_area(a, b, c);
    println!("area of triangle with sides: {a}, {b}, {c} = {area1}");
    let circumference = triangle_circumference(a, b, c);
    println!("circumference of triangle with sides: {a}, {b}, {c} = {circumference}");

    
}   

// takes as input line segments that form a triangle
fn can_create_triangle(a: f64, b: f64, c: f64) -> bool{
    return (a + b > c) && (a + c > b) && (b + c > a);
}

// takes as input line segments that form a triangle
fn triangle_area(a: f64, b: f64, c: f64) -> f64{
    return 0.25 * ((a + b + c) * (-a + b + c) * (a - b + c) * (a + b -c)).sqrt();
}

fn triangle_circumference(a: f64, b: f64, c: f64) -> f64{
    return a + b + c;
}

fn zadanie1(){

    assert_eq!(heaviside(5), 1);
    assert_eq!(heaviside(-5), 0);


    let x = 123;
    let y = 432;
    println!("swapping x: {x} with y: {y}.");
    
    let (new_x, new_y) = swap(x, y);

    println!("now x: {new_x}, y: {new_y}.");

    assert_eq!(2, abs(-2));

    assert_eq!(3, floor(3.14) as i64);
}

fn heaviside(x: i32) -> i32{
    if x < 0{
        return 0;
    }
    return 1;
}

fn swap(x: i32, y: i32) -> (i32, i32){
    return (y, x);
}

fn abs(x: i32) -> i32{
    return if x < 0 {-x} else {x};
}

fn floor(x: f64) -> f64{
    return x as f64 as f64;
}

