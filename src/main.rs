extern crate num;
mod bf;

use bf::bf::BruteForce;

fn main() {

    let mut br = BruteForce::default();

    br.set_char_range("11111".to_string()); 

    println!("{:?}",String::from_utf8_lossy(&br.char_range));

    br.pw_length = 3; 
    br.run_loops();

}
