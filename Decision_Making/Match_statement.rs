fn main(){
    let state_code="MH";
    let state =match state_code {
   "MH" =>{println!("Found match for MH");"Maharastra"},
   "KL"=>"Kerela",
    "KA"=>"Karnatka",
    "GA"=>"Goa",
    _ =>"Unkown"
    };
    println!("State name is {}",state);
}