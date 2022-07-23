fn main(){
    let tuple=(45,34.7,true);
    print(tuple);
}
fn print(x:(i32,f64,bool)){
    println!("{:?}",x);
}