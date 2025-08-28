use super::Token;

pub fn parse_int(lex: &mut logos::Lexer<'_, Token>) -> Option<i64> {
    lex.slice().parse::<i64>().ok()
}

pub fn parse_float(lex: &mut logos::Lexer<'_, Token>) -> Option<f64> {
    lex.slice().parse::<f64>().ok()
}

pub fn parse_string(lex: &mut logos::Lexer<'_, Token>) -> Option<String> {
    let s = lex.slice();
    Some(s[1..s.len() - 1].replace("\\\"", "\""))
}

pub fn parse_identifier(lex: &mut logos::Lexer<'_, Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

pub fn parse_line_comment(lex: &mut logos::Lexer<'_, Token>) -> Option<String> {
    let s = lex.slice();
    // Remove the // prefix
    Some(s[2..].to_string())
}

pub fn parse_block_comment(lex: &mut logos::Lexer<'_, Token>) -> Option<String> {
    let s = lex.slice();
    // Remove the /* */ wrapper
    Some(s[2..s.len() - 2].to_string())
}