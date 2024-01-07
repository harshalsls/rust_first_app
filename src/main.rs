#[allow(unused_variables)]
fn main() {
    println!("Hello, world! This is Harshal");
    let x = 1;

    let two: i32  = 2;
    let hello = "hello";
    let j = 'j';
    let my_half = 0.5;
    let my_name = "Bil";
    let quit_program = false;
    let your_half = my_half;

    let x = add(1,1);
    let y = add(3,0);
    let z = add(x, 1);
    println!("{}", z);
}

fn add(a: i32, b: i32) -> i32 {
    let x = (a + b)/2;
    return x;
}

