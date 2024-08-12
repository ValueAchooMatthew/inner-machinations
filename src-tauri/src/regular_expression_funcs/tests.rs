#[cfg(test)]
pub mod tests {
  use crate::regular_expression_funcs::{
    regex_models::{BinaryOperator, KleeneOperator, OrOperator, ParsingError, Token, UnaryOperator}, test_string_regex, build_parse_tree};

  #[test]
  fn test_parsing_or_operator() {
    
    let regex_to_test = "a + b";

    let expected_result = Token::OrOperator(Box::new(OrOperator::new(
    Some(Token::Literal(String::from("a"))), 
    Some(Token::Literal(String::from("b")))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_kleene_operator() {

    let regex_to_test = "a*";

    let expected_result = Token::KleeneOperator(Box::new(KleeneOperator::new(
      Some(Token::Literal(String::from("a")))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_literal() {
    let regex_to_test = "a";
    let expected_result = Token::Literal(String::from("a"));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_group_expression() {
    let regex_to_test = "(a)";
    let expected_result = Token::Literal(String::from("a"));
    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_all_operators() {
    let regex_to_test = "a+(a+b)*";
    // representing the (a+b)* part
    let inner_expression_as_token = Token::KleeneOperator(
      Box::new(KleeneOperator::new(
        Some(Token::OrOperator(Box::new(OrOperator::new(
          Some(Token::Literal(String::from("a"))), 
          Some(Token::Literal(String::from("b")))
        )))
      )))
    );

    let expected_result = Token::OrOperator(Box::new(OrOperator::new(
      Some(Token::Literal(String::from("a"))), 
      Some(inner_expression_as_token)
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

  }


  #[test]
  fn test_parsing_concatenation() {
    let regex_to_test = "a(a+b)*";
    // representing the (a+b)* part
    let inner_expression_as_token = Token::KleeneOperator(
      Box::new(KleeneOperator::new(
        Some(Token::OrOperator(Box::new(OrOperator::new(
          Some(Token::Literal(String::from("a"))), 
          Some(Token::Literal(String::from("b")))
        )))
      )))
    );

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::Literal(String::from("a")), 
      inner_expression_as_token.clone()
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "(a+b)*a";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      inner_expression_as_token,
      Token::Literal(String::from("a")) 
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "(a)(b)";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::Literal(String::from("a")),
      Token::Literal(String::from("b"))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "(a)(b)(c)";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::ConcatenatedExpression(Box::new((
        Token::Literal(String::from("a")),
        Token::Literal(String::from("b"))
      ))),
      Token::Literal(String::from("c"))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "a(b)*";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::Literal(String::from("a")),
      Token::KleeneOperator(Box::new(KleeneOperator::new(Some(Token::Literal(String::from("b")))
    ))))));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "a*b";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::KleeneOperator(Box::new(KleeneOperator::new(Some(Token::Literal(String::from("a")))))),
      Token::Literal(String::from("b"))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "(a+b)b";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::OrOperator(Box::new(OrOperator::new(
        Some(Token::Literal(String::from("a"))), 
        Some(Token::Literal(String::from("b")))))),
      Token::Literal(String::from("b"))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "(a(b))*";

    let expected_result = Token::KleeneOperator(Box::new(KleeneOperator::new(
      Some(Token::ConcatenatedExpression(Box::new((
        Token::Literal(String::from("a")),
        Token::Literal(String::from("b"))
      )))))));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

    let regex_to_test = "((a)(b))*";

    let expected_result = Token::KleeneOperator(Box::new(KleeneOperator::new(
      Some(
        Token::ConcatenatedExpression(Box::new((
          Token::Literal(String::from("a")),
          Token::Literal(String::from("b"))
        ))
      ))
    )));

    assert_eq!(expected_result, build_parse_tree(regex_to_test).unwrap());

  }

  #[test]
  fn test_parsing_invalid_kleene() {
    let regex_to_test = "*";

    let expected_result = ParsingError::NoInnerArg;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));

  }
  #[test]
  fn test_parsing_invalid_or() {
    let regex_to_test: &str = "a+";
    let expected_result = ParsingError::NoneTokenProvided;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));

    let regex_to_test: &str = "+b";
    let expected_result = ParsingError::NoneTokenProvided;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));
  }

  #[test]
  fn test_kleene_string_checking() {

    let regex_to_test: &str = "(abc)*";

    assert!(test_string_regex(regex_to_test, "".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abc".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abcabc".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abcabcabcabc".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()).unwrap());

    let regex_to_test: &str = "a*";

    assert!(test_string_regex(regex_to_test, "".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aa".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abcabcabca".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aaaaaaaaaab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "baaaaaaa".to_owned()).unwrap());
  }

  #[test]
  fn test_or_string_checking() {
    let regex_to_test: &str = "a+b";

    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
  }

  #[test]
  fn test_all_operators_string_checking() {

    let regex_to_test: &str = "(a+b)*";

    assert!(test_string_regex(regex_to_test, "".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "bbbbbbbbb".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aaaaaaaaa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abababababbaba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());

    let regex_to_test = "a+b*";

    assert!(test_string_regex(regex_to_test, "".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "bb".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "bbbbbbbbb".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abbbbbbbb".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());

  }

  #[test]
  fn test_concatenation_string_checking() {

    let regex_to_test = "(a)(b)";

    assert!(test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abab".to_owned()).unwrap());

    let regex_to_test = "a(a+b)";

    assert!(test_string_regex(regex_to_test, "aa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "bb".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aaa".to_owned()).unwrap());

    let regex_to_test = "(a+b)a";

    assert!(test_string_regex(regex_to_test, "aa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "bb".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aaa".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "bbb".to_owned()).unwrap());

    let regex_to_test = "a(a+b)*";

    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aaaa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abbbbb".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abbba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "babbabb".to_owned()).unwrap());

    let regex_to_test = "(a+b)*a";
    
    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ba".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aaaaa".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abbbba".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "abbaaba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "bab".to_owned()).unwrap());
    
    let regex_to_test = "((a)(b))*a";

    assert!(test_string_regex(regex_to_test, "a".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aba".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ababa".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aaaaa".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abbbba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abbaaba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "bab".to_owned()).unwrap());

    let regex_to_test = "((a+b)c)*";

    assert!(test_string_regex(regex_to_test, "".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ac".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "bc".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "acbc".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "bcac".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "abc".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ca".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "cb".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "cab".to_owned()).unwrap());

    let regex_to_test = "a*b";

    assert!(test_string_regex(regex_to_test, "b".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "ab".to_owned()).unwrap());
    assert!(test_string_regex(regex_to_test, "aaaaaaaaaaaaaaaaaaaaaaaaaab".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "c".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "ba".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "aaaaabb".to_owned()).unwrap());
    assert!(!test_string_regex(regex_to_test, "cab".to_owned()).unwrap());


  }

}