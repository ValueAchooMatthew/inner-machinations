#[cfg(test)]
pub mod tests {
  use crate::regular_expression_funcs::{regex_models::ParsingError, test_string_regex, build_parse_tree};


  // Commenting out parsing tests because in order for the string checking tests to function in the 
  // first place the parsing must be done correctly

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

    assert!(test_string_regex(regex_to_test, "".to_owned()));
    assert!(test_string_regex(regex_to_test, "abc".to_owned()));
    assert!(test_string_regex(regex_to_test, "abcabc".to_owned()));
    assert!(test_string_regex(regex_to_test, "abcabcabcabc".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()));

    let regex_to_test: &str = "a*";

    assert!(test_string_regex(regex_to_test, "".to_owned()));
    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "aa".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abcabcabca".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aaaaaaaaaab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "baaaaaaa".to_owned()));
  }

  #[test]
  fn test_or_string_checking() {
    let regex_to_test: &str = "a+b";

    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()));
  }

  #[test]
  fn test_all_operators_string_checking() {

    let regex_to_test: &str = "(a+b)*";

    assert!(test_string_regex(regex_to_test, "".to_owned()));
    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "b".to_owned()));
    assert!(test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(test_string_regex(regex_to_test, "bbbbbbbbb".to_owned()));
    assert!(test_string_regex(regex_to_test, "aaaaaaaaa".to_owned()));
    assert!(test_string_regex(regex_to_test, "abababababbaba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));

    let regex_to_test = "a+b*";

    assert!(test_string_regex(regex_to_test, "".to_owned()));
    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "b".to_owned()));
    assert!(test_string_regex(regex_to_test, "bb".to_owned()));
    assert!(test_string_regex(regex_to_test, "bbbbbbbbb".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abbbbbbbb".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));

  }

  #[test]
  fn test_concatenation_string_checking() {

    let regex_to_test = "(a)(b)";

    assert!(test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "a".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abab".to_owned()));

    let regex_to_test = "a(a+b)";

    assert!(test_string_regex(regex_to_test, "aa".to_owned()));
    assert!(test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "bb".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "a".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aaa".to_owned()));

    let regex_to_test = "(a+b)a";

    assert!(test_string_regex(regex_to_test, "aa".to_owned()));
    assert!(test_string_regex(regex_to_test, "ba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "bb".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "a".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aaa".to_owned()));
    assert!(!test_string_regex(regex_to_test, "bbb".to_owned()));

    let regex_to_test = "a(a+b)*";

    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(test_string_regex(regex_to_test, "aa".to_owned()));
    assert!(test_string_regex(regex_to_test, "aaaa".to_owned()));
    assert!(test_string_regex(regex_to_test, "abbbbb".to_owned()));
    assert!(test_string_regex(regex_to_test, "abbba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "babbabb".to_owned()));

    let regex_to_test = "(a+b)*a";
    
    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "ba".to_owned()));
    assert!(test_string_regex(regex_to_test, "aa".to_owned()));
    assert!(test_string_regex(regex_to_test, "aaaaa".to_owned()));
    assert!(test_string_regex(regex_to_test, "abbbba".to_owned()));
    assert!(test_string_regex(regex_to_test, "abbaaba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "bab".to_owned()));
    
    let regex_to_test = "((a)(b))*a";

    assert!(test_string_regex(regex_to_test, "a".to_owned()));
    assert!(test_string_regex(regex_to_test, "aba".to_owned()));
    assert!(test_string_regex(regex_to_test, "ababa".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aaaaa".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abbbba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abbaaba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "bab".to_owned()));

    let regex_to_test = "((a+b)c)*";

    assert!(test_string_regex(regex_to_test, "".to_owned()));
    assert!(test_string_regex(regex_to_test, "ac".to_owned()));
    assert!(test_string_regex(regex_to_test, "bc".to_owned()));
    assert!(test_string_regex(regex_to_test, "acbc".to_owned()));
    assert!(test_string_regex(regex_to_test, "bcac".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abc".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ca".to_owned()));
    assert!(!test_string_regex(regex_to_test, "cb".to_owned()));
    assert!(!test_string_regex(regex_to_test, "cab".to_owned()));

    let regex_to_test = "a*b";

    assert!(test_string_regex(regex_to_test, "b".to_owned()));
    assert!(test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(test_string_regex(regex_to_test, "aaaaaaaaaaaaaaaaaaaaaaaaaab".to_owned()));
    assert!(!test_string_regex(regex_to_test, "c".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ba".to_owned()));
    assert!(!test_string_regex(regex_to_test, "aaaaabb".to_owned()));
    assert!(!test_string_regex(regex_to_test, "cab".to_owned()));

    let regex_to_test = "ab + c";

    assert!(test_string_regex(regex_to_test, "ab".to_owned()));
    assert!(test_string_regex(regex_to_test, "c".to_owned()));
    assert!(!test_string_regex(regex_to_test, "".to_owned()));
    assert!(!test_string_regex(regex_to_test, "abc".to_owned()));
    assert!(!test_string_regex(regex_to_test, "a".to_owned()));
    assert!(!test_string_regex(regex_to_test, "b".to_owned()));
    assert!(!test_string_regex(regex_to_test, "ac".to_owned()));    

  }

}