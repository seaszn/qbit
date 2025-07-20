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