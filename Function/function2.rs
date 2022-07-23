fn main(){
  let mut no:i32=60;
  mutate_to_no_zero(&mut no);
  println!("{}",no);
}
fn mutate_to_no_zero(param_no:&mut i32){
    *param_no=0;
}
