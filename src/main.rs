extern crate num;

use num::pow;

#[derive(Default)]
struct BruteForce {
    char_range: Vec<u8>,
    pw_length: u8,
}

impl BruteForce {
    fn set_char_range(&mut self, mut input_count: String) {

        if input_count.pop().unwrap() == '1' {
            let mut char_range_special_1: Vec<u8> = (32..65).collect();
            self.char_range.append(&mut char_range_special_1);
        }

        if input_count.pop().unwrap() == '1' {
            let mut char_range_upper_case: Vec<u8> = (65..91).collect();
            self.char_range.append(&mut char_range_upper_case);
        }

        if input_count.pop().unwrap() == '1' {
            let mut char_range_special_2: Vec<u8> = (91..97).collect();
            self.char_range.append(&mut char_range_special_2);
        }

        if input_count.pop().unwrap() == '1' {
            let mut char_range_lower_case: Vec<u8> = (97..123).collect();
            self.char_range.append(&mut char_range_lower_case);
        }

        if input_count.pop().unwrap() == '1' {
            let mut char_range_special_3: Vec<u8> = (123..126).collect();
            self.char_range.append(&mut char_range_special_3);
        }

    }
    fn run_loops(&self) {

        // char range lenth
        let crl = self.char_range.len();

        if self.pw_length >= 1 {
            
            for i in self.char_range.clone() {
                self.try_current_pw(format!("{}", String::from_utf8_lossy(&[i])));
            }
            println!("tryed: {}",crl);
        }


        if self.pw_length >= 2 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    self.try_current_pw(format!("{}{}",
                                                String::from_utf8_lossy(&[i]),
                                                String::from_utf8_lossy(&[j])));
                }
            }
            println!("tryed: {}",pow(crl,2));
        }


        if self.pw_length >= 3 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    for k in self.char_range.clone() {
                        self.try_current_pw(format!("{}{}{}",
                                                    String::from_utf8_lossy(&[i]),
                                                    String::from_utf8_lossy(&[j]),
                                                    String::from_utf8_lossy(&[k])));
                    }
                }
            }
            println!("tryed: {}",pow(crl,3));
        }


        if self.pw_length >= 4 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    for k in self.char_range.clone() {
                        for l in self.char_range.clone() {
                            self.try_current_pw(format!("{}{}{}{}",
                                                        String::from_utf8_lossy(&[i]),
                                                        String::from_utf8_lossy(&[j]),
                                                        String::from_utf8_lossy(&[k]),
                                                        String::from_utf8_lossy(&[l])));
                        }
                    }
                }
            }
            println!("tryed: {}",pow(crl,4));
        }

        if self.pw_length >= 5 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    for k in self.char_range.clone() {
                        for l in self.char_range.clone() {
                            for m in self.char_range.clone() {
                                self.try_current_pw(format!("{}{}{}{}{}",
                                                            String::from_utf8_lossy(&[i]),
                                                            String::from_utf8_lossy(&[j]),
                                                            String::from_utf8_lossy(&[k]),
                                                            String::from_utf8_lossy(&[l]),
                                                            String::from_utf8_lossy(&[m])));
                            }
                        }
                    }
                }
            }
            println!("tryed: {}",pow(crl,5));
        }

        if self.pw_length >= 6 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    for k in self.char_range.clone() {
                        for l in self.char_range.clone() {
                            for m in self.char_range.clone() {
                                for n in self.char_range.clone() {
                                    self.try_current_pw(format!("{}{}{}{}{}{}",
                                                                String::from_utf8_lossy(&[i]),
                                                                String::from_utf8_lossy(&[j]),
                                                                String::from_utf8_lossy(&[k]),
                                                                String::from_utf8_lossy(&[l]),
                                                                String::from_utf8_lossy(&[m]),
                                                                String::from_utf8_lossy(&[n])));
                                }
                            }
                        }
                    }
                }
            }
            println!("tryed: {}",pow(crl,6));
        }

        if self.pw_length >= 7 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    for k in self.char_range.clone() {
                        for l in self.char_range.clone() {
                            for m in self.char_range.clone() {
                                for n in self.char_range.clone() {
                                    for o in self.char_range.clone() {
                                        self.try_current_pw(format!("{}{}{}{}{}{}{}",
                                                                    String::from_utf8_lossy(&[i]),
                                                                    String::from_utf8_lossy(&[j]),
                                                                    String::from_utf8_lossy(&[k]),
                                                                    String::from_utf8_lossy(&[l]),
                                                                    String::from_utf8_lossy(&[m]),
                                                                    String::from_utf8_lossy(&[n]),
                                                                    String::from_utf8_lossy(&[o])));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            println!("tryed: {}",pow(crl,7));
        }

        if self.pw_length == 8 {
            for i in self.char_range.clone() {
                for j in self.char_range.clone() {
                    for k in self.char_range.clone() {
                        for l in self.char_range.clone() {
                            for m in self.char_range.clone() {
                                for n in self.char_range.clone() {
                                    for o in self.char_range.clone() {
                                        for p in self.char_range.clone() {
                                        self.try_current_pw(format!("{}{}{}{}{}{}{}{}",
                                                                    String::from_utf8_lossy(&[i]),
                                                                    String::from_utf8_lossy(&[j]),
                                                                    String::from_utf8_lossy(&[k]),
                                                                    String::from_utf8_lossy(&[l]),
                                                                    String::from_utf8_lossy(&[m]),
                                                                    String::from_utf8_lossy(&[n]),
                                                                    String::from_utf8_lossy(&[o]),
                                                                    String::from_utf8_lossy(&[p])));
                                    }}
                                }
                            }
                        }
                    }
                }
            }
            println!("tryed: {}",pow(crl,8));
        }

        if self.pw_length > 8 {
            println!("zu lang");
        }

    }
    fn try_current_pw(&self, pw: String) {
        //print!("[{}]", pw);

        // testoperation, sould simulate an program call
        let test_operation = format!("{}{}",pw.clone(),"abc");
    }
}


fn main() {

    let mut br = BruteForce::default();

    br.set_char_range("01000".to_string()); 

    println!("{:?}",String::from_utf8_lossy(&br.char_range));

    br.pw_length = 5; 
    br.run_loops();

}
