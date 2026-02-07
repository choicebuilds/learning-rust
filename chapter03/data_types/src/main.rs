fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let x = 2.0; // f64 is the defauly
    let y: f32 = 3.0; // f32
    println!("{}", x + y);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (integerv, floatingv, unsignedv) = tup;
    println!("{integerv}, {floatingv}, {unsignedv}"); 
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5];
    println!("{}", a[0] + a[1] + a[2] + a[3] + a[4]);
}
