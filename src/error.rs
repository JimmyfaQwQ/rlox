pub fn error(line: usize, prefix: &str, message: &str) {
    report(line, &format!("{} Error", prefix), message);
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] {}: {}", line, location, message);
}

pub fn error_at_token(token: &crate::token::Token, prefix: &str, message: &str) {
    if token.token_type == crate::token::TokenType::EOF {
        report(token.line, &format!("{} at end", &format!("{} Error", prefix)), message);
    } else {
        report(token.line, &format!("{} at '{}'", &format!("{} Error", prefix), token.lexeme.as_ref().unwrap()), message);
    }
}