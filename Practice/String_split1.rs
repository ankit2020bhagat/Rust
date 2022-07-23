fn main(){
    let str1="ankit kumar bhagat";
    //println!("{}",str1.split(" "));
    let list:Vec<&str>=str1.split(" ").collect();
    println!("{}",list[0]);
    println!("{}",list[1]);
    println!("{}",list[2]);

}