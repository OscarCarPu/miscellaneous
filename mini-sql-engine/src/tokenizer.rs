#[derive(Debug, PartialEq)]
pub enum Token {
    Select,
    Star,
}

impl Token {
    pub fn tokenize(query: &str) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        for word in query.split_whitespace() {
            match word.to_uppercase().as_str() {
                "SELECT" => tokens.push(Token::Select),
                "*" => tokens.push(Token::Star),
                _ => return Err(format!("unknown term '{}'", word)),
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_select_star() {
        let result = Token::tokenize("SELECT *");

        assert_eq!(result, Ok(vec![Token::Select, Token::Star]));
    }

    #[test]
    fn tokenize_unkown_term() {
        assert!(Token::tokenize("insert").is_err());
    }
}
