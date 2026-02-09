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

// fn main() {
//     let s1: String = get_string();
//     println!("The value of s1 is {}", s1);

//     let s2: String = String::from("Krishna");
//     let s3: String = send_get_string(s2);

//     println!("The value of s3 is {}", s3);
// }

// fn get_string() -> String {
//     let new_string = String::from("Hare");
//     return new_string;
// }

// fn send_get_string(recieved_string: String) -> String {
//     return recieved_string;
// }


// fn main() {
//     let s1:String = String::from("Hare");
//     let len:usize = calculate_length(s1.clone());
//     println!("The value of {} is {}",s1,len);
// }

// fn calculate_length(s:String) -> usize {
//     return  s.len();
// }

// fn main() {
//     let s1:String = String::from("Hare");
//     let (s2,len) = calculate_length(s1);
//     println!("The value of {} is {}",s2,len);
// }

// fn calculate_length(s:String) -> (String,usize) {
//     let legth:usize = s.len();
//     return (s,legth);


// }

// fn main() {
//     let s1:String = String::from("Hare");
//     let len:usize = calculate_length(&s1);
//     println!("The value of {} is {}",s1,len);
// }

// fn calculate_length(s:&String) -> usize {
//     return  s.len();
// }


// fn main() {
//     let num:u8 = 22;
//     let y = &num;
//     println!("The value of y is {}",y);
// }

// fn main() {
//     let refenrence_to_nothing = create_string_ref();
//     // println!("The value of refenrence_to_nothing is {}",refenrence_to_nothing);
// }

// fn create_string_ref() -> &String {
//     let s = String::from("Hare");
//     return &s;
// }

// fn main() {
//     let arr1:[u8;5] = [1,2,3,4,5];
//     println!("The value of arr1 is {:?}",arr1);
//     println!("The length of arr1 is {}", arr1.len());
// }

// fn main() {
//     let arr: [&str;3] = ["Hare","Krishna","Rama"];
//     read_array(&arr);
//     println!("The value of arr is {:?}",arr);
// }

// fn read_array(arr2: &[&str;3]) {
//    println!("arr2 = {:?}",arr2);
// }

//  vector - Dynamic array

// fn main() {
//     // let mut v:Vec<i32> = Vec::new();
//     // let mut v = Vec::<i32>::new();
//     // v.push(10);
//     // v.push(20);
//     // v.push(30);

// let mut v = vec![10,20,30];
// v.push(100);  
// v.pop();
//     println!("The value of v is {:?}",v);
// }


// fn main() {
//     let mut vrr: Vec<&str> = vec!["Hare","Krishna","Rama"];
//     write_vrr(& mut vrr);  /*vrr ownership transfered*/
//     println!("The value of vrr is {:?}",vrr);
// }

// fn write_vrr(vrr2: & mut Vec<&str>) {
//     vrr2.push("Ram");
//     println!("The value of vrr2 is {:?}",vrr2);
// }

//  shadowing started

// fn main() {
//     let x = 5;
// println!("The value of x is {}",x);
//     let x = "Hare Krishna";
//     println!("The value of x is {}",x);
//     let x = x.len();
//     println!("The value of x is {}",x);

// }

//  if else statements

// fn main() {
//     let number = 12;

//     if number % 3 == 0 && number % 4 == 0 {
//         println!("The number is divisible by 3 and 4");
//     } else if number % 3 == 0 {
//         println!("The number is divisible by 3");
//     } else if number % 4 == 0 {
//         println!("The number is divisible by 4");
//     } else {
//         println!("The number is not divisible by 3 or 4");
//     }
// }


//  while loop statement
// fn main() {
//     let mut count = 0;
//      while count < 5 {
//         println!("The count is {}",count);
//         count += 1;
//      }
// }

//  for loop statement
// fn main() {
//     let arr = [10,20,30,40,50];
//     for arr in arr {
//         println!("The value of arr is {}",arr);
//     }
// }

//  match statement
// fn main() {
//     let number = 2;

//     match number {
//         1 => println!("The number is one"),
//         2 => println!("The number is two"),
//         3 => println!("The number is three"),
//         _ => println!("The number is not one, two, or three"),
//     };
// }

// fn main() {
//     let number = 100;

//     match number {
//       10 | 1 => println!("The number is one"),
//       20 |  2 => println!("The number is two and 20"),
//       30 |  3 => println!("The number is three"),
//         _ => println!("The number is not one, two, or three"),
//     };
// }


//  match statement with function
fn main() {
    fn is_even(num:i8)->bool{
        if num%2==0{
            return true;
        
    }
    return false;
    }
    let number = 1;

match number {
    1 | 3 => println!("The number is one or three"),
    2 | 4 => println!("The number is two or four"),
    5  => println!("The number is five"),
    _ => println!("The number is not one, two, or three"),
}
is_even(10);


}