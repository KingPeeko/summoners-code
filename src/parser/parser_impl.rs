use crate::lexer::tokens::*;
use crate::ast::*;

use super::token_stream::TokenStream;
use super::error::ParseError;


pub struct Parser<'a> {
    stream: TokenStream<'a>,
}

// Expect the given $enum from $stream
macro_rules! expect {
    ($stream:expr, $enum:pat) => {
        $stream.expect( |token| {
            matches!(token, $enum)
        })
    }
}

// Same as expect, but don't advance stream.
macro_rules! peek_validate {
    ($stream:expr, $enum:pat) => {
        {
            let token = $stream.peek();
            match token {
            Ok($enum) => Ok(token),
            Ok(_) => Err(ParseError::UnexpectedToken),
            Err(e) => Err(e),
            }
        }
    }
}

// Same as above, but peek n tokens ahead
macro_rules! peek_validate_ahead {
    ($stream:expr, $enum:pat, $n:expr) => {
        {
        let token = $stream.peek_ahead($n);
        match token {
            Ok($enum) => Ok(token),
            Ok(_) => Err(ParseError::UnexpectedToken),
            Err(e) => Err(e),
        }
        }
    }
}

// Expect $pattern from stream and unwrap its inner contents, otherwise return an Err type
macro_rules! expect_unwrap {
    ( $stream:expr, $pattern:path ) => {
        match $stream.peek() {
            Ok($pattern(x)) => {
                $stream.move_forward(1);
                Ok(x)
            }

            Ok(_) => Err(ParseError::UnexpectedToken),
            
            Err(e) => Err(e),
        }
    };
}

// Same as expect_unwrap, but don't advance the stream
macro_rules! peek_unwrap {
    ( $stream:expr, $pattern:path ) => {
        match $stream.peek() {
            Ok($pattern(x)) => Ok(x),

            Ok(_) => Err(ParseError::UnexpectedToken),

            Err(e) => Err(e),
        }
    }
}

// Same as above, but peek n ahead
macro_rules! peek_unwrap_ahead {
    ( $stream:expr, $pattern:path, $n:expr ) => {
        match $stream.peek_ahead($n) {
            Ok($pattern(x)) => Ok(x),

            Ok(_) => Err(ParseError::UnexpectedToken),

            Err(e) => Err(e),
        }
    }
}

type Err = ParseError;
impl<'a> Parser<'a> {


    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            stream: TokenStream::new(tokens),
        }
    }

    pub fn parse(mut self) -> Result<Program, Err> {
        todo!();
        let mut declarations = Vec::<Decl>::new();

        while !self.stream.is_eof() {
            declarations.push(self.parse_decl()?);
        }

        Ok(Program{ declarations })
    }

    fn parse_decl(&mut self) -> Result<Decl, Err> {
        todo!();
        let kw = expect_unwrap!(self.stream, Token::Keyword)?;

        match kw {
            Keyword::Ability => self.parse_ability(),
            Keyword::Buy => self.parse_item_decl(),
            Keyword::Nexus => self.parse_nexus(),

            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn parse_nexus(&mut self) -> Result<Decl, Err> {
        todo!();
    }

    fn parse_ability(&mut self) -> Result<Decl, Err> {
        todo!();
    }

    fn parse_item_decl(&mut self) -> Result<Decl, Err> {
        todo!();
        // Item decl needs tokens 'buy', 'identifier', 'colon', 'type', 'assignment', 'expression', 'semicolon'
        expect!(self.stream, Token::Keyword(Keyword::Buy))?;

        let variable_name = expect_unwrap!(self.stream, Token::Identifier)?;

        expect!(self.stream, Token::Symbol(Symbol::Colon))?;

        let ty = self.parse_type()?;

        expect!(self.stream, Token::Operator(Operator::Assignment))?;

        let expr = self.parse_expr()?;

        expect!(self.stream, Token::Symbol(Symbol::Semicolon))?;

        Ok(Decl::Item { 
            name: variable_name.to_string(), 
            ty,
            initializer: expr, 
        })
    }




    // Type is: 'type', and possibly 'AngleOpen', 'type', etc.
    fn parse_type(&mut self) -> Result<crate::ast::Type, Err> {
        let ty = peek_unwrap!(self.stream, Token::Type)?;
        match ty {
            // These functions want the Type token as well, so we don't advance the stream
            crate::lexer::Type::Inventory => self.parse_inventory_type(),
            crate::lexer::Type::Shop => self.parse_shop_type(),
            crate::lexer::Type::Duo => self.parse_duo_type(),

            x => {
                self.stream.move_forward(1);
                match x {
                    crate::lexer::Type::Gold => Ok(crate::ast::Type::Gold),
                    crate::lexer::Type::Chat => Ok(crate::ast::Type::Chat),
                    crate::lexer::Type::Void => Ok(crate::ast::Type::Void),
                    crate::lexer::Type::Status => Ok(crate::ast::Type::Status),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn parse_inventory_type(&mut self) -> Result<crate::ast::Type, Err> {
        expect!(self.stream, Token::Type(crate::lexer::Type::Inventory))?;
        expect!(self.stream, Token::Symbol(Symbol::AngleOpen))?;

        let ty = self.parse_type()?;

        expect!(self.stream, Token::Symbol(Symbol::AngleClose))?;

        Ok(crate::ast::Type::Inventory(Box::new(ty)))
    }

    fn parse_shop_type(&mut self) -> Result<crate::ast::Type, Err> {
        expect!(self.stream, Token::Type(crate::lexer::Type::Shop))?;
        expect!(self.stream, Token::Symbol(Symbol::AngleOpen))?;

        let ty1 = self.parse_type()?;
        expect!(self.stream, Token::Symbol(Symbol::Comma))?;
        let ty2 = self.parse_type()?;

        expect!(self.stream, Token::Symbol(Symbol::AngleClose))?;

        Ok(crate::ast::Type::Shop(Box::new(ty1), Box::new(ty2)))
    }

    fn parse_duo_type(&mut self) -> Result<crate::ast::Type, Err> {
        expect!(self.stream, Token::Type(crate::lexer::Type::Duo))?;
        expect!(self.stream, Token::Symbol(Symbol::AngleOpen))?;

        let ty1 = self.parse_type()?;
        expect!(self.stream, Token::Symbol(Symbol::Comma))?;
        let ty2 = self.parse_type()?;

        expect!(self.stream, Token::Symbol(Symbol::AngleClose))?;

        Ok(crate::ast::Type::Duo(Box::new(ty1), Box::new(ty2)))
    }

    fn parse_expr(&mut self) -> Result<Expr, Err> {
        todo!();
        // let token = self.stream.peek()?;
        //
        // match token {
        //     Token::Literal(_) => self.parse_lit(),
        //     Token::Identifier(s) => self.parse_identifier(),
        // }
    }


    fn parse_identifier(&mut self) -> Result<Expr, Err> {
        let ident_name = expect_unwrap!(self.stream, Token::Identifier)?;
        Ok(Expr::Identifier(ident_name.to_string()))
    }

    fn parse_lit(&mut self) -> Result<Expr, Err> {
        self.parse_status_lit()
            .or_else(|_| self.parse_void_lit())
            .or_else(|_| self.parse_gold_lit())
            .or_else(|_| self.parse_chat_lit())
    }

    fn parse_void_lit(&mut self) -> Result<Expr, Err> {
        // Check if next two tokens are '()'
        let _ = peek_validate!(self.stream, Token::Symbol(Symbol::ParenOpen))?;
        let _ = peek_validate_ahead!(self.stream, Token::Symbol(Symbol::ParenClose), 1)?;

        // If yes, advance two and return a unit expr
        self.stream.move_forward(2);

        Ok(Expr::Unit)
    }

    fn parse_status_lit(&mut self) -> Result<Expr, Err> {
        let kw = peek_unwrap!(self.stream, Token::Keyword)?;

        match kw {
            Keyword::True => {
                self.stream.move_forward(1);
                Ok(Expr::Boolean(true))
            }

            Keyword::False => {
                self.stream.move_forward(1);
                Ok(Expr::Boolean(false))
            }

            _ => Err(ParseError::ParseLitError),
        }
    }

    fn parse_chat_lit(&mut self) -> Result<Expr, Err> {
        let lit = peek_unwrap!(self.stream, Token::Literal)?;

        if let Literal::ChatLit(string) = lit {
            self.stream.move_forward(1);
            return Ok(Expr::String(string.to_string()))
        }

        Err(ParseError::UnexpectedToken)
    }

    fn parse_gold_lit(&mut self) -> Result<Expr, Err> {
        let lit = peek_unwrap!(self.stream, Token::Literal)?;

        if let Literal::GoldLit(num_str) = lit {
            let num = num_str.parse::<i64>().map_err(|_| ParseError::ParseLitError)?;
            // Advance stream, as we parse a token
            self.stream.move_forward(1);
            return Ok(Expr::Integer(num))
        }

        Err(ParseError::UnexpectedToken)
    }

    fn parse_grouped_expr(&mut self) -> Result<Expr, Err> {
        todo!();
        // expect!(self.stream, Token::Symbol(Symbol::ParenOpen))?;
        //
        // let expr = self.parse_expr()?;
        //
        // expect!(self.stream, Token::Symbol(Symbol::ParenClose))?;
        //
        // Ok(expr)
    }

    fn parse_binary_expr(&mut self) -> Result<Expr, Err> {
        todo!(); // needs parse_expr
        // In case completing the parse fails, save current position to return to later
        self.stream.save_pos();
        let expr1_res = self.parse_expr();

        let token_res = self.stream.advance();

        let expr2_res = self.parse_expr();

        let combined_res = {
            expr1_res.clone()
                .and(expr2_res.clone())
                .and(token_res.clone())
        };

        if let Err(e) = combined_res {
            // Parsing failed, load previous position before parsing tokens
            self.stream.load_pos();
            return Err(e)
        }

        // Just checked that none are Err type
        let expr1 = expr1_res.unwrap();
        let expr2 = expr2_res.unwrap();
        let token = token_res.unwrap();

        match token {
            Token::Symbol(sym) => {
                match sym {
                    Symbol::AngleOpen => {
                        Ok(
                            Expr::Binary { left: Box::new(expr1), operator: BinaryOp::Less, right: Box::new(expr2) }
                        )
                    }

                    Symbol::AngleClose => {
                        Ok(
                            Expr::Binary { left: Box::new(expr1), operator: BinaryOp::Greater, right: Box::new(expr2) }
                        )
                    }

                    _ => {
                        self.stream.load_pos();
                        Err(ParseError::UnexpectedToken)
                    }
                }
            }

            Token::Operator(op) => {
                Self::binary_op_token_to_expr(expr1, op, expr2)
            }

            _ => {
                self.stream.load_pos();
                Err(ParseError::UnexpectedToken)
            }

        }
    }

    fn binary_op_token_to_expr(left_expr: Expr, op_token: &Operator, right_expr: Expr) -> Result<Expr, Err> {
        let binary_op = match op_token {
            Operator::Plus => BinaryOp::Add,
            Operator::Minus => BinaryOp::Subtract,
            Operator::Mult => BinaryOp::Multiply,
            Operator::Divide => BinaryOp::Divide,
            Operator::Modulo => BinaryOp::Modulo,
            Operator::Equals => BinaryOp::Equal,
            Operator::NotEquals => BinaryOp::NotEqual,
            Operator::LessEquals => BinaryOp::LessEqual,
            Operator::GreaterEquals => BinaryOp::GreaterEqual,
            Operator::And => BinaryOp::And,
            Operator::Or => BinaryOp::Or,
            
            _ => return Err(ParseError::UnexpectedToken),
        };

        Ok(
            Expr::Binary { left: Box::new(left_expr), operator: binary_op, right: Box::new(right_expr) }
        )
    }


    // Parses the Inventory expression, aka. a list. Returns an Ok with the Inventory if syntax is correct.
    fn parse_inventory_expr(&mut self) -> Result<Expr, Err> {
        expect!(self.stream, Token::Symbol(Symbol::SquareOpen))?;

        let mut items = Vec::new();

        if let _ = peek_validate!(self.stream, Token::Symbol(Symbol::SquareClose)) {
            self.stream.move_forward(1);
            return Ok(Expr::Inventory(items));
        }

        loop {
            items.push(self.parse_expr()?);
            if let _ = peek_validate!(self.stream, Token::Symbol(Symbol::Comma)) {
                self.stream.move_forward(1);
            } else {
                break;
            }
        }

        expect!(self.stream, Token::Symbol(Symbol::SquareClose))?;

        return Ok(Expr::Inventory(items));
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::util::*;

    #[test]
    fn test_parse_type() {
        use crate::ast::*;
        let tokens = [
            ty("Shop"), sym("<"), ty("Duo"), sym("<"), ty("Chat"), sym(","), ty("Gold"),
            sym(">"), sym(","), ty("Inventory"), sym("<"), ty("Gold"), sym(">"), sym(">")
        ];

        let mut parser = Parser::new(&tokens);

        let should_be = Type::Shop(Box::new(Type::Duo(Box::new(Type::Chat), Box::new(Type::Gold))), Box::new(Type::Inventory(Box::new(Type::Gold))));
        assert_eq!(parser.parse_type().unwrap(), should_be );
    }

    #[test]
    fn test_parse_lit() {
        let tokens = [gold_lit(2938), chat_lit("hello world"), keyword("true"), sym("("), sym(")")];

        let mut parser = Parser::new(&tokens);


        let res1 = parser.parse_lit();
        let res2 = parser.parse_lit();
        let res3 = parser.parse_lit();
        let res4 = parser.parse_lit();
        let res5 = parser.parse_lit();

        assert_eq!(res1.unwrap(), Expr::Integer(2938));
        assert_eq!(res2.unwrap(), Expr::String("hello world".to_string()));
        assert_eq!(res3.unwrap(), Expr::Boolean(true));
        assert_eq!(res4.unwrap(), Expr::Unit);
        assert!(matches!(res5, Err(ParseError::Eof)));
    }

    #[test]
    fn test_parse_binary_expr() {
        todo!(); // Needs parse_expr first
        let tokens = [gold_lit(5000), op("*"), op("-"), gold_lit(1)];

        let mut parser = Parser::new(&tokens);
        let result = parser.parse_binary_expr().unwrap();

        let should_be = Expr::Binary { left: Box::new(Expr::Integer(5000)), operator: BinaryOp::Multiply, right: Box::new(Expr::Unary { operator: UnaryOp::Negate, right: Box::new(Expr::Integer(1)) }) };

        assert_eq!(result, should_be);
    }

}
