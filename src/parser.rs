 use crate::lexer::{C1Lexer, C1Token};
 use crate::ParseResult;

 use std::ops::{Deref, DerefMut};

 pub struct C1Parser<'a>(C1Lexer<'a>);


 impl<'a> Deref for C1Parser<'a> {
     type Target = C1Lexer<'a>;

     fn deref(&self) -> &Self::Target {
         &self.0
     }
 }

 impl<'a> DerefMut for C1Parser<'a> {
     fn deref_mut(&mut self) -> &mut Self::Target {
         &mut self.0
     }
 }

 impl<'a> C1Parser<'a> {
     pub fn parse(text: &str) -> ParseResult {
         let mut parser = Self::initialize_parser(text);
         parser.program()
     }

     fn initialize_parser(text: &str) -> C1Parser {
         C1Parser(C1Lexer::new(text))
     }
//
//     /// program ::= ( functiondefinition )* <EOF>
     fn program(&mut self) -> ParseResult {
        while self.current_token().is_some() {
            self.functiondefinition()?;
        }
        Ok(())

     }

     fn functiondefinition(&mut self) -> ParseResult {
        self.return_type()?;
      let _ =  self.check_and_eat_token(&C1Token::Identifier,"kein Identifier");
      let _ =  self.check_and_eat_token(&C1Token::LeftParenthesis,"LeftParenthesis fehlt");
      let _ =  self.check_and_eat_token(&C1Token::RightParenthesis,"RightParenthesis fehlt");
      let _ =  self.check_and_eat_token(&C1Token::LeftBrace,"LeftBrace fehlt");
        self.statement_list()?;
        self.check_and_eat_token(&C1Token::RightBrace,"RightBrace fehlt")
     }
     
    fn return_type(&mut self) -> ParseResult {
        return match self.current_token() {
            Some(C1Token::KwBoolean) | Some(C1Token::KwFloat) | Some(C1Token::KwInt) | Some(C1Token::KwVoid) => {
                self.eat();
                Result::Ok(())
            },
            _ => Result::Err(self.error_message_current("Fehler")),
        };
    }


    fn statement_list(&mut self) -> ParseResult {

        loop {
            match self.current_token() {
                Some(C1Token::LeftBrace) | Some(C1Token::KwIf) | Some(C1Token::KwReturn) | Some(C1Token::KwPrintf) | Some(C1Token::Identifier) => {
                  self.block()?;
                }
                Some(_) | None => break,
            }
        }
        Ok(())
    }

    fn block(&mut self) -> ParseResult {
        if self.current_matches(&C1Token::LeftBrace){
            self.eat();
            self.statement_list()?;
            self.check_and_eat_token(&C1Token::RightBrace,"RightBrace fehlt")

        }else{
            self.statement()
        }
        
    }

    fn statement(&mut self) -> ParseResult {
        match self.current_token() {
            Some(C1Token::KwIf) => return self.if_statement(),
            Some(C1Token::KwReturn) => {
                self.return_statement()?;
                return self.check_and_eat_token(&C1Token::Semicolon,"Semicolon fehlt");},
            Some(C1Token::KwPrintf) => {
                self.printf()?; 
                return self.check_and_eat_token(&C1Token::Semicolon,"Semicolon fehlt");},
            Some(C1Token::Identifier) => {
                if self.next_matches(&C1Token::LeftParenthesis) {
                    self.function_call()?;
                    return self.check_and_eat_token(&C1Token::Semicolon,"Semicolon fehlt");
                } else if self.next_matches(&C1Token::Assign) {
                    self.stat_assignment()?;
                    return self.check_and_eat_token(&C1Token::Semicolon,"Semicolon fehlt");
                }
                return Result::Err(self.error_message_current("fehler"));
            },
            _ => return Result::Err(self.error_message_current("Fehler")),
        };

    }


    fn if_statement(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::KwIf,"KWif fehlt")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis,"LeftParenthesis fehlt")?;
        self.assignment()?;
        self.check_and_eat_token(&C1Token::RightParenthesis,"Right parenthesiss fehlt")?;
        self.block()
    }

    fn return_statement(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::KwReturn,"Kwreturn fehlt")?;
        if self.current_matches(&C1Token::Semicolon) {
           Ok(())
        }else{
           self.assignment() 
        }
        
       
    }

   

    fn printf(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::KwPrintf,"KwPrintf fehlt")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis,"Leftparenthesis fehlt")?;
        self.assignment()?;
        self.check_and_eat_token(&C1Token::RightParenthesis,"Rightparenthesis fehlt")
    }

    fn stat_assignment(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::Identifier,"identifier fehlt")?;
        self.check_and_eat_token(&C1Token::Assign,"Assign fehlt")?;
        self.assignment()
    }



    fn function_call(&mut self) -> ParseResult {
        self.check_and_eat_token(&C1Token::Identifier,"identifier fehlt")?;
        self.check_and_eat_token(&C1Token::LeftParenthesis,"Leftparenthesis fehlt")?;
        self.check_and_eat_token(&C1Token::RightParenthesis,"Rightparenthesis fehlt")
    }

    fn assignment(&mut self) -> ParseResult {
       
      
        if self.current_matches(&C1Token::Identifier) && self.next_matches(&C1Token::Assign){
              self.eat();
              self.eat();
            self.assignment() 
               
        }else{
           self.expr()
        }
        
    }

    fn expr(&mut self) -> ParseResult {
        self.simpexpr()?;

        if self.current_matches(&C1Token::Equal) || self.current_matches(&C1Token::NotEqual) || self.current_matches(&C1Token::LessEqual) || self.current_matches(&C1Token::GreaterEqual) || self.current_matches(&C1Token::Less) || self.current_matches(&C1Token::Greater){
            self.eat();
          //  let _ =    
            self.simpexpr()
        }else{
            Ok(())  
        }
      
        
        
    }



    fn simpexpr(&mut self) -> ParseResult {
        if self.current_token() == Some(C1Token::Minus) {
            self.eat();
        }
        self.term()?;
        loop {
            match self.current_token() {
                Some(C1Token::Plus) | Some(C1Token::Minus) | Some(C1Token::Or) => {
                    self.eat();
                    self.term()?;
                }
                Some(_) | None => break,
            }
        }

        Ok(())
    }


    fn term(&mut self) -> ParseResult {
        self.factor()?;
        loop {
            match self.current_token() {
                Some(C1Token::Asterisk) | Some(C1Token::Slash) | Some(C1Token::And) => {
                    self.eat();
                    self.factor()?;
                }
                Some(_) | None => break,
            }
        }

        Ok(())
    }



    fn factor(&mut self) -> ParseResult {


        if self.current_matches(&C1Token::LeftParenthesis){
            self.eat();
            self.assignment()?;
            self.check_and_eat_token(&C1Token::RightParenthesis, &self.error_message_current("assignment in factor wasn't properly closed. Expected )"))
        }
        else if self.current_matches(&C1Token::Identifier)&&self.next_matches(&C1Token::LeftParenthesis){
            self.function_call()
        }
        else if self.current_matches(&C1Token::Identifier) | self.current_matches(&C1Token::ConstBoolean) | self.current_matches(&C1Token::ConstFloat)  |  self.current_matches(&C1Token::ConstInt) {
           self.eat();
           Ok(())
        
        }

        else{
             return Result::Err(self.error_message_current("Fehlt"))

        } 



    }




     fn check_and_eat_token(&mut self, token: &C1Token, error_message: &str) -> ParseResult {
         if self.current_matches(token) {
             self.eat();
             Ok(())
         } else {
             Err(String::from(error_message))
         }
     }

     fn current_matches(&self, token: &C1Token) -> bool {
         match &self.current_token() {
             None => false,
             Some(current) => current == token,
         }
     }

     fn next_matches(&self, token: &C1Token) -> bool {
         match &self.peek_token() {
             None => false,
             Some(next) => next == token,
         }
     }
     
     fn error_message_current(&self, reason: &'static str) -> String {
        match self.current_token() {
            None => format!("{}. Reached EOF", reason),
            Some(_) => format!(
                "{} at line {:?} with text: '{}'",
                reason,
                self.current_line_number().unwrap(),
                self.current_text().unwrap()
            ),
        }
    }
  }  

#[cfg(test)]
mod tests {
    use crate::parser::{C1Parser, ParseResult};

    fn call_method<'a, F>(parse_method: F, text: &'static str) -> ParseResult
    where
        F: Fn(&mut C1Parser<'a>) -> ParseResult,
    {
        let mut parser = C1Parser::initialize_parser(text);
        if let Err(message) = parse_method(&mut parser) {
            eprintln!("Parse Error: {}", message);
            Err(message)
        } else {
            Ok(())
        }
    }

    #[test]
    fn parse_empty_program() {
        let result = C1Parser::parse("");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("   ");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("// This is a valid comment!");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("/* This is a valid comment!\nIn two lines!*/\n");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("  \n ");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn fail_invalid_program() {
        let result = C1Parser::parse("  bool  ");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("int x = 0;");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("// A valid comment\nInvalid line.");
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn valid_function() {
        let result = C1Parser::parse("  void foo() {}  ");
        assert!(result.is_ok());

        let result = C1Parser::parse("int bar() {return 0;}");
        assert!(result.is_ok());

        let result = C1Parser::parse(
            "float calc() {\n\
        x = 1.0;
        y = 2.2;
        return x + y;
        \n\
        }",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn fail_invalid_function() {
        let result = C1Parser::parse("  void foo()) {}  ");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("const bar() {return 0;}");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse(
            "int bar() {
                                                          return 0;
                                                     int foo() {}",
        );
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse(
            "float calc(int invalid) {\n\
        int x = 1.0;
        int y = 2.2;
        return x + y;
        \n\
        }",
        );
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn valid_function_call() {
        assert!(call_method(C1Parser::function_call, "foo()").is_ok());
        assert!(call_method(C1Parser::function_call, "foo( )").is_ok());
        assert!(call_method(C1Parser::function_call, "bar23( )").is_ok());
    }

    #[test]
    fn fail_invalid_function_call() {
        assert!(call_method(C1Parser::function_call, "foo)").is_err());
        assert!(call_method(C1Parser::function_call, "foo{ )").is_err());
        assert!(call_method(C1Parser::function_call, "bar _foo( )").is_err());
    }

    #[test]
    fn valid_statement_list() {
        assert!(call_method(C1Parser::statement_list, "int x = 4;").is_ok());
        assert!(call_method(
            C1Parser::statement_list,
            "int x = 4;\n\
        int y = 2.1;"
        )
        .is_ok());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
        {\
        foo();\n\
        }"
        )
        .is_ok());
        assert!(call_method(C1Parser::statement_list, "{x = 4;}\nint y = 1;\nfoo;\n{}").is_ok());
    }

    #[test]
    fn fail_invalid_statement_list() {
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4\n\
        y = 2.1;"
        )
        .is_err());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
        {\
        foo();"
        )
        .is_err());
        assert!(call_method(C1Parser::statement_list, "{x = 4;\ny = 1;\nfoo;\n{}").is_err());
    }

    #[test]
    fn valid_if_statement() {
        assert!(call_method(C1Parser::if_statement, "if(x == 1) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(x == y) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(z) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(true) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(false) {}").is_ok());
    }

    #[test]
    fn fail_invalid_if_statement() {
        assert!(call_method(C1Parser::if_statement, "if(x == ) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if( == y) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if(> z) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if( {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if(false) }").is_err());
    }

    #[test]
    fn valid_return_statement() {
        assert!(call_method(C1Parser::return_statement, "return x").is_ok());
        assert!(call_method(C1Parser::return_statement, "return 1").is_ok());
        assert!(call_method(C1Parser::return_statement, "return;").is_ok());
    }

    #[test]
    fn fail_invalid_return_statement() {
        assert!(call_method(C1Parser::return_statement, "1").is_err());
    }

    #[test]
    fn valid_printf_statement() {
        assert!(call_method(C1Parser::printf, " printf(a+b)").is_ok());
        assert!(call_method(C1Parser::printf, "printf( 1)").is_ok());
        assert!(call_method(C1Parser::printf, "printf(a - c)").is_ok());
    }

    #[test]
    fn fail_invalid_printf_statement() {
        assert!(call_method(C1Parser::printf, "printf( ").is_err());
        assert!(call_method(C1Parser::printf, "printf(printf)").is_err());
        assert!(call_method(C1Parser::printf, "Printf()").is_err());
    }

    #[test]
    fn valid_return_type() {
        assert!(call_method(C1Parser::return_type, "void").is_ok());
        assert!(call_method(C1Parser::return_type, "bool").is_ok());
        assert!(call_method(C1Parser::return_type, "int").is_ok());
        assert!(call_method(C1Parser::return_type, "float").is_ok());
    }

    #[test]
    fn valid_assignment() {
        assert!(call_method(C1Parser::assignment, "x = y").is_ok());
        assert!(call_method(C1Parser::assignment, "x =y").is_ok());
        assert!(call_method(C1Parser::assignment, "1 + 2").is_ok());
    }

    #[test]
    fn valid_stat_assignment() {
        assert!(call_method(C1Parser::stat_assignment, "x = y").is_ok());
        assert!(call_method(C1Parser::stat_assignment, "x =y").is_ok());
        assert!(call_method(C1Parser::stat_assignment, "x =y + t").is_ok());
    }

    #[test]
    fn valid_factor() {
        assert!(call_method(C1Parser::factor, "4").is_ok());
        assert!(call_method(C1Parser::factor, "1.2").is_ok());
        assert!(call_method(C1Parser::factor, "true").is_ok());
        assert!(call_method(C1Parser::factor, "foo()").is_ok());
        assert!(call_method(C1Parser::factor, "x").is_ok());
        assert!(call_method(C1Parser::factor, "(x + y)").is_ok());
    }

    #[test]
    fn fail_invalid_factor() {
        assert!(call_method(C1Parser::factor, "if").is_err());
        assert!(call_method(C1Parser::factor, "(4").is_err());
        assert!(call_method(C1Parser::factor, "bool").is_err());
    }

    #[test]
    fn multiple_functions() {
        assert!(call_method(
            C1Parser::program,
            "void main() { hello();}\nfloat bar() {return 1.0;}"
        )
        .is_ok());
    }
}
