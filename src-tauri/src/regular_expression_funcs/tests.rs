#[cfg(test)]
pub mod tests {
  use crate::regular_expression_funcs::{interpret_regex, regex_models::{BinaryOperator, KleeneOperator, OrOperator, ParsingError, Token, UnaryOperator}, test_string_regex};

  #[test]
  fn test_parsing_or_operator() {
    
    let regex_to_test = "a + b";

    let expected_result = Token::OrOperator(Box::new(OrOperator::new(
    Some(Token::Literal(String::from("a"))), 
    Some(Token::Literal(String::from("b")))
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_kleene_operator() {

    let regex_to_test = "a*";

    let expected_result = Token::KleeneOperator(Box::new(KleeneOperator::new(
      Some(Token::Literal(String::from("a")))
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_literal() {
    let regex_to_test = "a";
    let expected_result = Token::Literal(String::from("a"));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());
  }

  #[test]
  fn test_parsing_group_expression() {
    let regex_to_test = "(a)";
    let expected_result = Token::Literal(String::from("a"));
    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());
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

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

  }

  #[test]
  fn test_parsing_invalid_kleene() {
    let regex_to_test = "*";

    let expected_result = ParsingError::NoInnerArg;

    assert_eq!(Err(expected_result), interpret_regex(regex_to_test));

  }
  #[test]
  fn test_parsing_invalid_or() {
    let regex_to_test: &str = "a+";
    let expected_result = ParsingError::NoRightArg;

    assert_eq!(Err(expected_result), interpret_regex(regex_to_test));
  }

  #[test]
  fn test_kleene_string_checking() {
    let regex_to_test: &str = "(abc)*";

    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.clone(), "".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abc".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abcabc".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abcabcabcabcabcabcabcabc".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "ab".to_owned()));

  }

  #[test]
  fn test_or_string_checking() {
    let regex_to_test: &str = "a+b";

    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.clone(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "ab".to_owned()));

  }

  #[test]
  fn test_all_operators_string_checking() {
    let regex_to_test: &str = "(a+b)*";

    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.clone(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "b".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "ab".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "bbbbbbbbb".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "aaaaaaaaa".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abababababbabababbbababababba".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "c".to_owned()));

  }


}