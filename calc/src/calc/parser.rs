

use crate::calc::{Token, TokenType};



//to evaluate expressions that contain parenthesis as well
pub fn eval_with_parenthesis(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;
    if tokens.iter().any(|token| 
        matches!(
            token.token_type,
            TokenType::LeftParen | TokenType::RightParen
        )
    ) {
        let mut x = 0;
        while x < tokens.len() {
            if tokens[x].token_type == TokenType::LeftParen {
                let mut no_right_paren = true;
                let mut y = x + 1;
                while y < tokens.len() {
                    if tokens[y].token_type == TokenType::RightParen {
                        no_right_paren = false;
                        break;
                    }
                    y += 1;
                }

                if no_right_paren {
                    return Err("invalid expression!".to_string());
                } else if tokens[x + 1..y]
                    .iter()
                    .any(|token| token.token_type == TokenType::LeftParen)
                {
                    x += 1;
                    continue;
                } else if y - x == 1 {
                        //empty parentheses
                        return Err("invalid expression!".to_string());
                } else {
                        let paren_expression = tokens[x + 1..y].to_vec();
                        let paren_exp_result =  eval(paren_expression)?;

                        if x > 0 && y+1 < tokens.len() && tokens[x-1].token_type == TokenType::Num && tokens[y+1].token_type == TokenType::Num {

                            tokens.drain(x..=y);
                            tokens.insert(x, Token { token_type: TokenType::Multiply, value: "*".to_string() });
                            tokens.insert(x+1, paren_exp_result[0].clone());
                            tokens.insert(x+2,Token { token_type: TokenType::Multiply, value: "*".to_string() });
                        
                            return eval_with_parenthesis(tokens.clone());
                        }else if x > 0 && tokens[x-1].token_type == TokenType::Num {
                            tokens.drain(x..=y);
                            tokens.insert(x, Token { token_type: TokenType::Multiply, value: "*".to_string() });
                            tokens.insert(x+1, paren_exp_result[0].clone());

                            return eval_with_parenthesis(tokens.clone());
                            
                        }else if y+1 < tokens.len() && tokens[y+1].token_type == TokenType::Num {
                            tokens.drain(x..=y);
                            tokens.insert(x, paren_exp_result[0].clone());
                            tokens.insert(x+1, Token { token_type: TokenType::Multiply, value: "*".to_string() });
                        
                            return eval_with_parenthesis(tokens.clone());
                        }else {
                            tokens.drain(x..=y);
                            tokens.insert(x,paren_exp_result[0].clone());

                            return eval_with_parenthesis(tokens.clone());
                        }
                            
                        
                        
                }
                
            }
            x += 1;
        }
    }

    eval(tokens)
}



//evaluates the expression in the form of Vec<token> that can contain addition, subtraction, multiplication, division and remainder
fn eval(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;

    if tokens.len() == 1 && tokens[0].token_type == TokenType::Num {
        Ok(tokens)
    } else if tokens
        .iter()
        .any(|token| matches!(token.token_type, TokenType::Plus | TokenType::Minus))
    {
        let mut result_tokens: Vec<Token> = vec![];
        let mut y = 0;
        while y < tokens.len() {
            if y != 0 && tokens[y].token_type == TokenType::Plus
                || tokens[y].token_type == TokenType::Minus
            {
                let temp_tokens = tokens[0..y].to_vec();
                match multiply_and_divide_and_remainder(temp_tokens) {
                    Ok(result) => {
                        result_tokens.push(result[0].clone());
                    }
                    Err(err) => return Err(err),
                }
                result_tokens.push(tokens[y].clone());
                tokens.drain(0..=y);
                y = 0;
                continue;
            }
            y += 1;
        }

        if !tokens.is_empty() {
            match multiply_and_divide_and_remainder(tokens) {
                Ok(result) => {
                    result_tokens.push(result[0].clone());
                }
                Err(err) => return Err(err),
            }
        }

        //return result
        add_and_subtract(result_tokens)
    } else if tokens.iter().any(|token| {
        matches!(
            token.token_type,
            TokenType::Multiply | TokenType::Divide | TokenType::Remainder
        )
    }) {
        //return result
        multiply_and_divide_and_remainder(tokens)
    } else {
        return Err("invalid expression!".to_string());
    }
}

fn add_and_subtract(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;
    let mut x = 0;
    while x < tokens.len() {
        if tokens.len() == 1 && tokens[x].token_type == TokenType::Num {
            return Ok(tokens);
        } else if x != 0 && x + 1 < tokens.len() && tokens[x].token_type == TokenType::Plus || tokens[x].token_type == TokenType::Minus {
            let left_operand = match tokens[x - 1].value.parse::<f64>() {
                Ok(num) => num,
                Err(_) => return Err("invalid expression!".to_string()),
            };
            let right_operand = match tokens[x + 1].value.parse::<f64>() {
                Ok(num) => num,
                Err(_) => return Err("invalid expression!".to_string()),
            };

            let mut result = 0.0;
            if tokens[x].token_type == TokenType::Plus {
                result = left_operand + right_operand;
            } else if tokens[x].token_type == TokenType::Minus {
                result = left_operand - right_operand;
                
            }
            let result_token = Token {
                token_type: TokenType::Num,
                value: result.to_string(),
            };
            tokens.drain(x - 1..=x + 1);
            tokens.insert(x - 1, result_token);
            x -= 1;
            continue;
        } else if tokens[x].token_type == TokenType::Num {
            x += 1;
            continue;
        } else {
            return Err("invalid expression!".to_string());
        }
    }

    if tokens.len() == 1 && tokens[x].token_type == TokenType::Num {
        Ok(tokens)
    } else {
        Err("invalid expression!".to_string())
    }
}

fn multiply_and_divide_and_remainder(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut tokens = tokens;
    let mut x = 0;
    while x < tokens.len() {
        if tokens.len() == 1 && tokens[x].token_type == TokenType::Num {
            return Ok(tokens);
        } else if x != 0 && x + 1 < tokens.len() && tokens[x].token_type == TokenType::Multiply || tokens[x].token_type == TokenType::Divide || tokens[x].token_type == TokenType::Remainder {
            let left_operand = match tokens[x - 1].value.parse::<f64>() {
                Ok(num) => num,
                Err(_) => return Err("invalid expression!".to_string()),
            };
            let right_operand = match tokens[x + 1].value.parse::<f64>() {
                Ok(num) => num,
                Err(_) => return Err("invalid expression!".to_string()),
            };

            let mut result= 0.0;
            if tokens[x].token_type == TokenType::Multiply {
                
                result = left_operand * right_operand;
            }else if tokens[x].token_type == TokenType::Divide {
                result = left_operand / right_operand;
                
            }else if tokens[x].token_type == TokenType::Remainder {
                result = left_operand % right_operand;
                
            }
            let result_token = Token {
                token_type: TokenType::Num,
                value: result.to_string(),
            };
            tokens.drain(x - 1..=x + 1);
            tokens.insert(x - 1, result_token);
            x -= 1;
            continue;
        } else if tokens[x].token_type == TokenType::Num {
            x += 1;
            continue;
        } else {
            return Err("invalid expression!".to_string());
        }
    }

    if tokens.len() == 1 && tokens[x].token_type == TokenType::Num {
        Ok(tokens)
    } else {
        Err("invalid expression!".to_string())
    }
}