fn main(){
    let str1="geeks for geeks is best for geek".to_string();
    let mut i=0;
    for token in str1.split_whitespace(){
        println!("{} {} ",i,token);
        i+=1;
    }
}