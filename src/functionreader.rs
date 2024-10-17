use std::num;

enum Operand {
    PLUS,
    MINUS,
    TIMES,
    TIMESA,
    EXPONENT,
    DIVIDE,
}

pub fn read_function<'a>(function: &[char], input: f64) -> Result<f64, String>{

    //find the first component of an operation
    let first_operator = match function[0] {
        
        //if it's an open bracket, we find the corresponding closing bracket and process the subfunction within
        '(' => {
            let c = bracket_parse(&function).unwrap();
            (read_function(&function[1..c+1], input).unwrap(), c)
        },

        //this part looks for single-argument operators, or functions, which are
        //(in this program) limited to sqrt, sin, cos, tan, and abs. more may be
        //added later
        's'|'c'|'t'|'a' => {
            if function[1..6] == ['q','r','t','('] {
                let c = bracket_parse(&function[4..]).unwrap();
                (f64::sqrt(read_function(&function[4..c], input).unwrap()), c)

            }else if function[1..4] == ['i','n','('] {
                let c = bracket_parse(&function[3..]).unwrap() + 3;
                (f64::sin(read_function(&function[3..c+1],input).unwrap()), c)
            
            }else if function[1..4] == ['o','s','('] {
                let c = bracket_parse(&function[3..]).unwrap();
                (f64::cos(read_function(&function[3..c],input).unwrap()), c)
            
            }else if function[1..4] == ['a','n','('] {
                let c = bracket_parse(&function[3..]).unwrap();
                (f64::tan(read_function(&function[3..c],input).unwrap()), c)

            }else if function[1..4] == ['b','s','('] {
                let c = bracket_parse(&function[3..]).unwrap();
                (f64::abs(read_function(&function[3..c],input).unwrap()), c)
            }
            
            else{return Result::Err("error: function not recognized. This program
            only recognizes sqrt(), sin(), cos(), tan(), and abs()".to_string())}
        }

        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'.' => {
            number_parse(&function).unwrap()
        }

        'x'=>(input,1),
        
        _ => return Result::Err("starting character '".to_owned() + &function[0].to_string() + &"' not recognized")
    };

    if first_operator.1 == function.len() - 1 {return Ok(first_operator.0)}

    let operand = match function[first_operator.1] {
        '+'=>Operand::PLUS,
        '-'=>Operand::MINUS,
        '/'=>Operand::DIVIDE,
        '*'=>Operand::TIMESA,
        '^'=>Operand::EXPONENT,
        _=>Operand::TIMES,
    };

    let second_operator = read_function(&function[first_operator.1+match operand {Operand::TIMES=>0,_=>1}..], input).unwrap();

    return Ok(match operand {
        Operand::PLUS => first_operator.0 + second_operator,
        Operand::MINUS => first_operator.0 - second_operator,
        Operand::DIVIDE => first_operator.0 / second_operator,
        Operand::EXPONENT => first_operator.0.powf(second_operator),
        _=> first_operator.0 * second_operator,
    });



    //return Ok(first_operator.0);
    return Result::Err("unidentified error".to_string())
}

pub fn bracket_parse<'a>(function: &[char]) -> Result<usize, &'a str>{
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

pub fn number_parse<'a>(function: &[char]) -> Result<(f64, usize), &'a str>{
    let mut full_number: f64 = 0.0;
    let mut decimal: Option<i32> = None;

    let mut c: usize = 0;
    for _ in 0..function.len(){
        match function[c].to_digit(10){
            
            Some(digit) => {
                full_number += digit as f64 / (10.0 as f64).powf(c as f64 - (if decimal.is_some(){1.0}else{0.0}))
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
                    full_number *= (10.0 as f64).powf(match decimal{None=>c as i32,Some(i)=>i} as f64 - 1.0);
                    return Result::Ok((full_number, c))
                }
            }
        }
    c+=1;
    }

    full_number *= (10.0 as f64).powf(match decimal{None=>c as i32,Some(i)=>i} as f64 - 1.0);
    return Result::Ok((full_number, c));
}