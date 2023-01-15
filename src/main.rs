fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 let     exists: bool = std::path::Path::new(".\\vector.txt").exists();
 let mut input : String = String::new()                               ;

 if exists {
  input = std::fs::read_to_string(".\\vector.txt").expect("Failed file reading!");

 }//if exists {

 loop {
  if exists {
   if input.trim().is_empty() {
    println!("\r\n\r\ninput:");

    input = request();
   }//if input.trim().is_empty() {

  } else {//if exists {
   println!("\r\n\r\ninput:");

   input = request();
  }//} else {//if exists {

  if &input[..] == "exit" {
   break;   

  } else {//if &input[..] == "exit" {
   let mut integer: Vec<i32>      = serde_json::from_str(&input[..]).expect("Wrong format");
   let mut size   : usize         = integer.len()                                          ;
   let mut zero   : Vec<Vec<i32>> = vec![]                                                 ;

   integer.sort();

   if size > 2usize {
    let n: usize = size - 2usize;

    for i in 0..n {
     let mut j: usize = i + 1usize;
     let mut k: usize = n + 1usize;

     while j < k {
      let sum: i32 = integer[i] + integer[j] + integer[k];

      if sum == 0 {
       zero.push([integer[i], integer[j], integer[k]].to_vec());

       j += 1;
       k -= 1;

      } else {//if sum == 0 {
       if sum < 0 {
        j += 1;

       } else {//if sum < 0 {
        k -= 1;

       }//} else {//if sum < 0 {
      }//} else {//} else {//if sum == 0 {
     }//while j < k {
    }//for i in 0..n {
    
    zero.sort();

    size = zero.len();

    if size > 1 {
     let last: usize = size - 1usize;

     for i in (0..last).rev() {
      let j: usize = i + 1usize;

      if zero[i][0] == zero[j][0] && zero[i][1] == zero[j][1] && zero[i][2] == zero[j][2] {
       zero.remove(j);

      }//if zero[i][0] == zero[j][0] && zero[i][1] == zero[j][1] && zero[i][2] == zero[j][2] {
     }//for i in (0..last).rev() {
    }//if size > 1 {
   }//if size > 2usize {

   println!("\r\nzero:\r\n{:?}", zero);
  }//} else {//if &input[..] == "exit" {

  if exists {
   break;

  }//if exists {
 }//loop {
}//fn main() {
