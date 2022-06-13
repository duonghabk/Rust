
//use std::io;
//use rand::Rng;

fn main() {
    let org_arr = [1, 2,3,5,6,6,8,10,11];
    let sub_arr = [6,8,10];
    println!("org_arr : {:?}",org_arr);
    println!("sub_arr : {:?}",sub_arr);
    let mut ans = 0;
    for i in 0..org_arr.len(){
        if org_arr[i]== sub_arr[0]{  
            let mut j = 1;         
            while j<sub_arr.len() {
                if org_arr[i+j] != sub_arr[j]{
                    break;
                }
               j+=1;
            }
            if j== sub_arr.len(){
                ans = 1;
            }
        }
    }
    if ans == 1{
        println!("sub_arr la day con cua org_arr");
    }
    else{
        println!("sub_arr khong la day con cua org_arr");
    }
    
}
