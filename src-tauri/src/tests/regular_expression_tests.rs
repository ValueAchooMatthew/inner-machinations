#[cfg(test)]
pub mod tests {
  use crate::regular_expression_funcs::{regular_expression_parsing::build_parse_tree, regular_expression_models::ParsingError, 
  regular_expression_linguistics::test_string_regex};

  #[test]
  fn test_parsing_invalid_kleene() {
    let regex_to_test = "*";
    let expected_result = ParsingError::NoInnerArg;
    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));
  }

  #[test]
  fn test_parsing_invalid_or() {
    let regex_to_test: &str = "a+";
    let expected_result = ParsingError::EmptyRightArg;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));

    let regex_to_test: &str = "+b";
    let expected_result = ParsingError::EmptyLeftArg;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));

    let regex_to_test: &str = "a*+";
    let expected_result = ParsingError::EmptyRightArg;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));

    let regex_to_test: &str = "+b*";
    let expected_result = ParsingError::EmptyLeftArg;

    assert_eq!(Err(expected_result), build_parse_tree(regex_to_test));

  }

  #[test]
  fn test_kleene_string_checking() {

    let regex_to_test: &str = "(abc)*";

    assert!(test_string_regex(regex_to_test, ""));
    assert!(test_string_regex(regex_to_test, "abc"));
    assert!(test_string_regex(regex_to_test, "abcabc"));
    assert!(test_string_regex(regex_to_test, "abcabcabcabc"));
    assert!(!test_string_regex(regex_to_test, "ab"));

    let regex_to_test: &str = "a*";

    assert!(test_string_regex(regex_to_test, ""));
    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "aa"));
    assert!(!test_string_regex(regex_to_test, "abcabcabca"));
    assert!(!test_string_regex(regex_to_test, "aaaaaaaaaab"));
    assert!(!test_string_regex(regex_to_test, "baaaaaaa"));
  }

  #[test]
  fn test_or_string_checking() {
    let regex_to_test: &str = "a+b";

    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "ab"));
  }

  #[test]
  fn test_all_operators_string_checking() {

    let regex_to_test: &str = "(a+b)*";

    assert!(test_string_regex(regex_to_test, ""));
    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "b"));
    assert!(test_string_regex(regex_to_test, "ab"));
    assert!(test_string_regex(regex_to_test, "bbbbbbbbb"));
    assert!(test_string_regex(regex_to_test, "aaaaaaaaa"));
    assert!(test_string_regex(regex_to_test, "abababababbaba"));
    assert!(!test_string_regex(regex_to_test, "c"));

    let regex_to_test = "a+b*";

    assert!(test_string_regex(regex_to_test, ""));
    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "b"));
    assert!(test_string_regex(regex_to_test, "bb"));
    assert!(test_string_regex(regex_to_test, "bbbbbbbbb"));
    assert!(!test_string_regex(regex_to_test, "ab"));
    assert!(!test_string_regex(regex_to_test, "abbbbbbbb"));
    assert!(!test_string_regex(regex_to_test, "c"));

  }

  #[test]
  fn test_concatenation_string_checking() {

    let regex_to_test = "(a)(b)";

    assert!(test_string_regex(regex_to_test, "ab"));
    assert!(!test_string_regex(regex_to_test, "a"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "c"));
    assert!(!test_string_regex(regex_to_test, "abab"));

    let regex_to_test = "a(a+b)";

    assert!(test_string_regex(regex_to_test, "aa"));
    assert!(test_string_regex(regex_to_test, "ab"));
    assert!(!test_string_regex(regex_to_test, "bb"));
    assert!(!test_string_regex(regex_to_test, "ba"));
    assert!(!test_string_regex(regex_to_test, "a"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "aba"));
    assert!(!test_string_regex(regex_to_test, "abab"));
    assert!(!test_string_regex(regex_to_test, "c"));
    assert!(!test_string_regex(regex_to_test, "aaa"));

    let regex_to_test = "(a+b)a";

    assert!(test_string_regex(regex_to_test, "aa"));
    assert!(test_string_regex(regex_to_test, "ba"));
    assert!(!test_string_regex(regex_to_test, "bb"));
    assert!(!test_string_regex(regex_to_test, "ab"));
    assert!(!test_string_regex(regex_to_test, "a"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "abab"));
    assert!(!test_string_regex(regex_to_test, "aba"));
    assert!(!test_string_regex(regex_to_test, "c"));
    assert!(!test_string_regex(regex_to_test, "aaa"));
    assert!(!test_string_regex(regex_to_test, "bbb"));

    let regex_to_test = "a(a+b)*";

    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "ab"));
    assert!(test_string_regex(regex_to_test, "aa"));
    assert!(test_string_regex(regex_to_test, "aaaa"));
    assert!(test_string_regex(regex_to_test, "abbbbb"));
    assert!(test_string_regex(regex_to_test, "abbba"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "ba"));
    assert!(!test_string_regex(regex_to_test, "babbabb"));

    let regex_to_test = "(a+b)*a";
    
    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "ba"));
    assert!(test_string_regex(regex_to_test, "aa"));
    assert!(test_string_regex(regex_to_test, "aaaaa"));
    assert!(test_string_regex(regex_to_test, "abbbba"));
    assert!(test_string_regex(regex_to_test, "abbaaba"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "ab"));
    assert!(!test_string_regex(regex_to_test, "bab"));
    
    let regex_to_test = "((a)(b))*a";

    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "aba"));
    assert!(test_string_regex(regex_to_test, "ababa"));
    assert!(!test_string_regex(regex_to_test, "aaaaa"));
    assert!(!test_string_regex(regex_to_test, "abbbba"));
    assert!(!test_string_regex(regex_to_test, "abbaaba"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "ab"));
    assert!(!test_string_regex(regex_to_test, "bab"));

    let regex_to_test = "((a+b)c)*";

    assert!(test_string_regex(regex_to_test, ""));
    assert!(test_string_regex(regex_to_test, "ac"));
    assert!(test_string_regex(regex_to_test, "bc"));
    assert!(test_string_regex(regex_to_test, "acbc"));
    assert!(test_string_regex(regex_to_test, "bcac"));
    assert!(!test_string_regex(regex_to_test, "c"));
    assert!(!test_string_regex(regex_to_test, "abc"));
    assert!(!test_string_regex(regex_to_test, "ca"));
    assert!(!test_string_regex(regex_to_test, "cb"));
    assert!(!test_string_regex(regex_to_test, "cab"));

    let regex_to_test = "a*b";

    assert!(test_string_regex(regex_to_test, "b"));
    assert!(test_string_regex(regex_to_test, "ab"));
    assert!(test_string_regex(regex_to_test, "aaaaaaaaaaaaaaaaaaaaaaaaaab"));
    assert!(!test_string_regex(regex_to_test, "c"));
    assert!(!test_string_regex(regex_to_test, "ba"));
    assert!(!test_string_regex(regex_to_test, "aaaaabb"));
    assert!(!test_string_regex(regex_to_test, "cab"));

    let regex_to_test = "ab + c";

    assert!(test_string_regex(regex_to_test, "ab"));
    assert!(test_string_regex(regex_to_test, "c"));
    assert!(!test_string_regex(regex_to_test, ""));
    assert!(!test_string_regex(regex_to_test, "abc"));
    assert!(!test_string_regex(regex_to_test, "a"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "ac"));

    let regex_to_test = "a + bc*";

    assert!(test_string_regex(regex_to_test, ""));
    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "bc"));
    assert!(test_string_regex(regex_to_test, "bcbcbcbcbcbcbcbc"));
    assert!(!test_string_regex(regex_to_test, "abc"));
    assert!(!test_string_regex(regex_to_test, "b"));
    assert!(!test_string_regex(regex_to_test, "ac"));
    assert!(!test_string_regex(regex_to_test, "cb"));

    let regex_to_test = "a + b(c*)";

    assert!(test_string_regex(regex_to_test, "a"));
    assert!(test_string_regex(regex_to_test, "b"));
    assert!(test_string_regex(regex_to_test, "bc"));
    assert!(test_string_regex(regex_to_test, "bccccccccccccccccccccccccccccc"));
    assert!(!test_string_regex(regex_to_test, ""));
    assert!(!test_string_regex(regex_to_test, "abc"));
    assert!(!test_string_regex(regex_to_test, "ac"));
    assert!(!test_string_regex(regex_to_test, "cb"));

  }

}