use crate::{ParseResult, C1Lexer, C1Token};

pub struct C1Parser;

/*
pub fn check_and_eat_token (token: C1Token,mut lexer: C1Lexer)-> ParseResult {
    

    let lineN =  lexer.current_line_number().unwrap();
            let textE = format!("Syn Error at line {}", lineN);

    if current_matches(token , lexer){
        lexer.eat();
    }
    
    let lineN =  lexer.current_line_number().unwrap();
    let textE = format!("Syn Error at line {}", lineN);

    match lexer.current_token() {
        Some(token) => Ok(()),
        None => Err(textE),
    }
    
}*/


impl C1Parser{
    pub fn parse(text: &str) -> ParseResult {
        /*let mut lexer = C1Lexer::new(text);
        let lineN =  lexer.current_line_number().unwrap();
        let textE = format!("Syn Error at line {}", lineN);

        while lexer.current_text() != None{   
            let lineN =  lexer.current_line_number().unwrap();
            
            match lexer.current_token().unwrap(){

                KwBoolean  => {
                    lexer.eat()


                }
                
                _ => { return Err(textE);
                }
                 

            }
            
            
            

        }*/
       
        
    Ok(())

    }

    pub fn current_matches (token: C1Token, lexer: &C1Lexer) -> bool{
        return Some(token) == lexer.current_token();
    
    }
    
    pub fn next_matches (token: C1Token,lexer: &C1Lexer) -> bool{
        return Some(token) == lexer.peek_token();
    
    }

    

    


}