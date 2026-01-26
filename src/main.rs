// fn main() {
//     print!("The sum is {}",sum(10,20))
// }

// fn sum(a: i32, b: i32) -> i32 {
//   let result =   a + b;
//   return result;
// }

// fn main() {
//     let mut num:u8 = 22;
//     println!("The number is {}",num);
//     num = 4;
//     println!("The number is {}",num);
// }
// fn main() {
//     let mut str:String = String::from("Hare Krishna ");
//     str.push_str("Hare Krishna Krishna Hare Hare");
//     println!(" {}",str);
// }

// fn main() {
//     println!("{}", sum(10, 20))
// }

// fn sum(a: u8, b: u8) -> u8 {
//     let result = a + b;
//     return result;
// }

// fn main() {
//     let a = 5;
//     let b = a;
//     println!("a={}",a);
//     println!("b={}",b);
//     println!("a={}",a);

// }

// fn main() {
//     let str1 = String::from("Hare Krishna");
//     let str2 = str1;
//     // println!("str1={}",str1);
//     println!("str2={}",str2);
// }

// fn main() {
//     let x:u8 = 5;
//     process_integer(x);
//     println!("The value of x is {}",x);
// }

// fn process_integer(x:u8) {
//     println!("The value of x in process_integer is {}",x);
// }

// fn main() {
//     let x:String = String::from("Hare Krishna");
//     process_string(x);
//     // println!("The value of x is {}",x);
// }

// fn process_string(item:String) {
//     println!("The value of x in process_integer is {}",item);
// }

fn main() {
    let s1: String = get_string();
    println!("The value of s1 is {}", s1);

    let s2: String = String::from("Krishna");
    let s3: String = send_get_string(s2);

    println!("The value of s3 is {}", s3);
}

fn get_string() -> String {
    let new_string = String::from("Hare");
    return new_string;
}

fn send_get_string(recieved_string: String) -> String {
    return recieved_string;
}
