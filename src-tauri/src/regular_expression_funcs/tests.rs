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

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    let regex_to_test = "(a+b)*a";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      inner_expression_as_token,
      Token::Literal(String::from("a")) 
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    let regex_to_test = "(a)(b)";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::Literal(String::from("a")),
      Token::Literal(String::from("b"))
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    let regex_to_test = "(a)(b)(c)";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::ConcatenatedExpression(Box::new((
        Token::Literal(String::from("a")),
        Token::Literal(String::from("b"))
      ))),
      Token::Literal(String::from("c"))
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    let regex_to_test = "a(b)*";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::Literal(String::from("a")),
      Token::KleeneOperator(Box::new(KleeneOperator::new(Some(Token::Literal(String::from("b")))
    ))))));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    let regex_to_test = "a*b";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::KleeneOperator(Box::new(KleeneOperator::new(Some(Token::Literal(String::from("a")))))),
      Token::Literal(String::from("b"))
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    let regex_to_test = "(a+b)b";

    let expected_result = Token::ConcatenatedExpression(Box::new((
      Token::OrOperator(Box::new(OrOperator::new(
        Some(Token::Literal(String::from("a"))), 
        Some(Token::Literal(String::from("b")))))),
      Token::Literal(String::from("b"))
    )));

    assert_eq!(expected_result, interpret_regex(regex_to_test).unwrap());

    // Problem is that currently, a regex that looks like a(b)*
    // and ((a)(b))* are treated the same, due to the fact grouped expressions are always
    // Broken down by the compiler into ab* (with kleene working on the nearest token, that being 'b')
    // To try and fix this I should try reworking the compiler such that operators take on the nearest grouped expression
    // As their argument, (and making an expression a grouped expression if none exist)
    // And then expanding out grouped expressions within the parse tree rather than prior to insertion

    // A final parse tree should never consist of grouped expressions, but inserting should (maybe?)

    // IMPORTANT: Consider a(b)* case vs (a(b))* case (one should become ab* the other should become (ab*))

    let regex_to_test = "((a)(b))*";

    let expected_result = Token::KleeneOperator(Box::new(KleeneOperator::new(
      Some(
        Token::ConcatenatedExpression(Box::new((
          Token::Literal(String::from("a")),
          Token::Literal(String::from("b"))
        ))
      ))
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
    let expected_result = ParsingError::EmptyRightArg;

    assert_eq!(Err(expected_result), interpret_regex(regex_to_test));
  }

  #[test]
  fn test_kleene_string_checking() {
    let regex_to_test: &str = "(abc)*";

    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.clone(), "".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abc".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abcabc".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abcabcabcabc".to_owned()));

    assert!(!test_string_regex(parse_tree.clone(), "ab".to_owned()));

    let regex_to_test: &str = "a*";
    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.clone(), "".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "aa".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "abcabcabca".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "aaaaaaaaaab".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "baaaaaaa".to_owned()));

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

    assert!(test_string_regex(parse_tree.clone(), "".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "b".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "ab".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "bbbbbbbbb".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "aaaaaaaaa".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "abababababbaba".to_owned()));

    assert!(!test_string_regex(parse_tree.clone(), "c".to_owned()));

    let regex_to_test = "a+b*";
    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.clone(), "".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "b".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "bb".to_owned()));
    assert!(test_string_regex(parse_tree.clone(), "bbbbbbbbb".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "ab".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "abbbbbbbb".to_owned()));
    assert!(!test_string_regex(parse_tree.clone(), "c".to_owned()));

  }

  #[test]
  fn test_concatenation_string_checking() {
    let regex_to_test = "(a)(b)";

    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.to_owned(), "ab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "a".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "c".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "abab".to_owned()));

    let regex_to_test = "a(a+b)";

    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.to_owned(), "aa".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "ab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "bb".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "ba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "a".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "aba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "abab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "c".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "aaa".to_owned()));

    let regex_to_test = "(a+b)a";
    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.to_owned(), "aa".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "ba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "bb".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "ab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "a".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "abab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "aba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "c".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "aaa".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "bbb".to_owned()));

    let regex_to_test = "a(a+b)*";
    let parse_tree = interpret_regex(regex_to_test).unwrap();

    assert!(test_string_regex(parse_tree.to_owned(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "ab".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "aa".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "aaaa".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "abbbbb".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "abbba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "ba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "babbabb".to_owned()));


    let regex_to_test = "(a+b)*a";
    let parse_tree = interpret_regex(regex_to_test).unwrap();
    
    assert!(test_string_regex(parse_tree.to_owned(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "ba".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "aa".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "aaaaa".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "abbbba".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "abbaaba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "ab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "bab".to_owned()));
    
    
    // Need to fix for this test case
    let regex_to_test = "((a)(b))*a";

    let parse_tree = interpret_regex(regex_to_test).unwrap();
    
    assert!(test_string_regex(parse_tree.to_owned(), "a".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "aba".to_owned()));
    assert!(test_string_regex(parse_tree.to_owned(), "ababa".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "aaaaa".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "abbbba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "abbaaba".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "b".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "ab".to_owned()));
    assert!(!test_string_regex(parse_tree.to_owned(), "bab".to_owned()));

  }

}