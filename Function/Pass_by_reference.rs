fn main(){
    let mut num=54;
    println!("{}",num);
    pass_by_refernce(&mut num);
    println!("{}",num);
}

fn pass_by_refernce(num:&mut i32){
    *num=65;
}