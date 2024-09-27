use std::num;

pub fn read_function<'a>(function: &[char], input: f32) -> Result<f32, &'a str>{

    //find the first component of an operation
    let first_operator = match function[0] {
        
        //if it's an open bracket, we find the corresponding closing bracket and process the subfunction within
        '(' => {
            let c = bracket_parse(&function).unwrap();
            read_function(&function[1..c], input).unwrap()
        },

        //this part looks for single-argument operators, or functions, which are
        //(in this program) limited to sqrt, sin, cos, and tan.
        's'|'c'|'t' => {
            if function[1..5] == ['q','r','t','('] {
                let c = bracket_parse(&function[5..]).unwrap();
                f32::sqrt(read_function(&function[5..c], input).unwrap())

            }else if function[1..3] == ['i','n','('] {
                let c = bracket_parse(&function[4..]).unwrap();
                f32::sin(read_function(&function[4..c],input).unwrap())
            
            }else if function[1..3] == ['o','s','('] {
                let c = bracket_parse(&function[4..]).unwrap();
                f32::cos(read_function(&function[4..c],input).unwrap())
            
            }else if function[1..3] == ['a','n','('] {
                let c = bracket_parse(&function[4..]).unwrap();
                f32::tan(read_function(&function[4..c],input).unwrap())
            }
            
            else{return Result::Err("error: function not recognized. This program
            only recognizes sqrt(), sin(), cos() and tan()")}
        }
        
        _ => return Result::Err("starting character not recognized")        
    };

    return Result::Err("unidentified error")
}

fn bracket_parse<'a>(function: &[char]) -> Result<usize, &'a str>{
    let mut bracket_number = 0;
    for c in 0..function.len() {
        
        if function[c]==')'{
            bracket_number-=1;
        }
        
        else if function[c]=='('{
            bracket_number+=1;
        }
        
        if bracket_number==0{
            return Result::Ok(c)
        }
    
    }return Result::Err("no closing paratheses!")
}

pub fn number_parse<'a>(function: &[char]) -> Result<(f32, usize), &'a str>{
    let mut full_number: f32 = 0.0;
    let mut decimal: Option<i32> = None;

    let mut c: usize = 0;
    for _ in 0..function.len(){
        match function[c].to_digit(10){
            
            Some(digit) => {
                full_number += digit as f32 / (10.0 as f32).powf(c as f32 - (if decimal.is_some(){1.0}else{0.0}))
            }

            None =>  {
                //we have located the decimal point of the number
                if function[c]=='.'{

                    //more than one decimal point???? heresy!
                    if decimal.is_some() {return Result::Err("cannot have more than one decimal place in a single literal number")}
                    decimal = Some(c as i32);
                }

                //we have found the end of the number
                else {
                    full_number *= (10.0 as f32).powf(match decimal{None=>1,Some(i)=>i} as f32 - 1.0);
                    return Result::Ok((full_number, c))
                }
            }
        }
    c+=1;
    }

    full_number *= (10.0 as f32).powf(match decimal{None=>1,Some(i)=>i} as f32 - 1.0);
    return Result::Ok((full_number, c));
}