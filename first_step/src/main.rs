fn main() {

    let number:u8 =100;
    let slice = "Hello!";
    let slice2 = "안녕!";

    println!("The first number is {}, slice has {} bytes but also {} characters", number, slice.len(), slice.chars().count());
    println!("Slice2 has {} bytes but only {} characters", slice2.len(), slice2.chars().count());
}
