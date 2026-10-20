use std::collections::BTreeSet;

use crate::regular_expression_funcs::{regex_models::{ParsingError, Token}, test_string_regex, parse_regular_expression, EMPTY_STRING};

fn accepts(regex: &str, string_to_check: &str) -> bool {
  test_string_regex(regex, string_to_check.to_owned()).unwrap()
}

#[test]
fn test_parsing_invalid_kleene() {
  assert_eq!(Err(ParsingError::NoInnerArg), parse_regular_expression("*"));
  assert_eq!(Err(ParsingError::NoInnerArg), parse_regular_expression("*a"));
  assert_eq!(Err(ParsingError::NoInnerArg), parse_regular_expression("(*)"));
  // Previously passed the verifier and then panicked while building the NFA
  assert_eq!(Err(ParsingError::NoInnerArg), parse_regular_expression("a+*"));
}

#[test]
fn test_parsing_invalid_or() {
  assert_eq!(Err(ParsingError::EmptyRightArg), parse_regular_expression("a+"));
  assert_eq!(Err(ParsingError::EmptyLeftArg), parse_regular_expression("+b"));
  assert_eq!(Err(ParsingError::EmptyRightArg), parse_regular_expression("a++b"));
  assert_eq!(Err(ParsingError::EmptyLeftArg), parse_regular_expression("a(+b)"));
  assert_eq!(Err(ParsingError::EmptyRightArg), parse_regular_expression("a|"));
}

#[test]
fn test_parsing_invalid_brackets_and_empty_expressions() {
  // An unmatched closing bracket used to cut the regex off silently
  assert_eq!(Err(ParsingError::UnmatchedClosingBracket), parse_regular_expression("a)b"));
  assert_eq!(Err(ParsingError::UnmatchedClosingBracket), parse_regular_expression(")"));
  assert_eq!(Err(ParsingError::MissingClosingBracket), parse_regular_expression("(a"));
  assert_eq!(Err(ParsingError::MissingClosingBracket), parse_regular_expression("((a)"));
  assert_eq!(Err(ParsingError::EmptyExpression), parse_regular_expression(""));
  assert_eq!(Err(ParsingError::EmptyExpression), parse_regular_expression("  "));
  assert_eq!(Err(ParsingError::EmptyBrackets), parse_regular_expression("()"));
  assert_eq!(Err(ParsingError::EmptyBrackets), parse_regular_expression("a()b"));
  assert_eq!(Err(ParsingError::MissingClosingBracket), parse_regular_expression("("));
}

#[test]
fn test_parse_tree_shape() {
  // The frontend draws this tree, so check the serialized form
  let parse_tree = parse_regular_expression("a+b*").unwrap();
  let json = serde_json::to_value(&parse_tree).unwrap();
  assert_eq!(json["OrOperator"]["left_argument"]["Literal"], "a");
  assert_eq!(json["OrOperator"]["right_argument"]["KleeneOperator"]["inner_argument"]["Literal"], "b");
  assert_eq!(json["OrOperator"]["operator_character"], "+");

  let parse_tree = parse_regular_expression("abc").unwrap();
  let json = serde_json::to_value(&parse_tree).unwrap();
  assert_eq!(json["ConcatenatedExpression"]["right_argument"]["Literal"], "c");
  assert_eq!(json["ConcatenatedExpression"]["left_argument"]["ConcatenatedExpression"]["left_argument"]["Literal"], "a");
}

#[test]
fn test_kleene_string_checking() {

  let regex_to_test: &str = "(abc)*";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "abc"));
  assert!(accepts(regex_to_test, "abcabc"));
  assert!(accepts(regex_to_test, "abcabcabcabc"));
  assert!(!accepts(regex_to_test, "ab"));

  let regex_to_test: &str = "a*";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "aa"));
  assert!(!accepts(regex_to_test, "abcabcabca"));
  assert!(!accepts(regex_to_test, "aaaaaaaaaab"));
  assert!(!accepts(regex_to_test, "baaaaaaa"));

  let regex_to_test: &str = "a**";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "aaa"));
  assert!(!accepts(regex_to_test, "b"));
}

#[test]
fn test_or_string_checking() {
  let regex_to_test: &str = "a+b";

  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "ab"));

  // | is accepted as union too
  assert!(accepts("a|b", "b"));
  assert!(!accepts("a|b", "ab"));
}

#[test]
fn test_union_binds_looser_than_concatenation() {
  let regex_to_test: &str = "ab+c";

  assert!(accepts(regex_to_test, "ab"));
  assert!(accepts(regex_to_test, "c"));
  assert!(!accepts(regex_to_test, "ac"));
  assert!(!accepts(regex_to_test, "abc"));

  let regex_to_test: &str = "c+(ab)*";

  assert!(accepts(regex_to_test, "c"));
  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "abab"));
  assert!(!accepts(regex_to_test, "abc"));
  assert!(!accepts(regex_to_test, "cab"));
}

#[test]
fn test_nested_kleene_string_checking() {
  let regex_to_test: &str = "((ab)*c)*";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "c"));
  assert!(accepts(regex_to_test, "abc"));
  assert!(accepts(regex_to_test, "ababcc"));
  assert!(accepts(regex_to_test, "cabc"));
  assert!(!accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "abcab"));
  assert!(!accepts(regex_to_test, "acb"));
}

#[test]
fn test_all_operators_string_checking() {

  let regex_to_test: &str = "(a+b)*";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "b"));
  assert!(accepts(regex_to_test, "ab"));
  assert!(accepts(regex_to_test, "bbbbbbbbb"));
  assert!(accepts(regex_to_test, "aaaaaaaaa"));
  assert!(accepts(regex_to_test, "abababababbaba"));
  assert!(!accepts(regex_to_test, "c"));

  let regex_to_test = "a+b*";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "b"));
  assert!(accepts(regex_to_test, "bb"));
  assert!(accepts(regex_to_test, "bbbbbbbbb"));
  assert!(!accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "abbbbbbbb"));
  assert!(!accepts(regex_to_test, "c"));

}

#[test]
fn test_concatenation_string_checking() {

  let regex_to_test = "(a)(b)";

  assert!(accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "a"));
  assert!(!accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "c"));
  assert!(!accepts(regex_to_test, "abab"));

  let regex_to_test = "a(a+b)";

  assert!(accepts(regex_to_test, "aa"));
  assert!(accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "bb"));
  assert!(!accepts(regex_to_test, "ba"));
  assert!(!accepts(regex_to_test, "a"));
  assert!(!accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "aba"));
  assert!(!accepts(regex_to_test, "abab"));
  assert!(!accepts(regex_to_test, "c"));
  assert!(!accepts(regex_to_test, "aaa"));

  let regex_to_test = "(a+b)a";

  assert!(accepts(regex_to_test, "aa"));
  assert!(accepts(regex_to_test, "ba"));
  assert!(!accepts(regex_to_test, "bb"));
  assert!(!accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "a"));
  assert!(!accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "abab"));
  assert!(!accepts(regex_to_test, "aba"));
  assert!(!accepts(regex_to_test, "c"));
  assert!(!accepts(regex_to_test, "aaa"));
  assert!(!accepts(regex_to_test, "bbb"));

  let regex_to_test = "a(a+b)*";

  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "ab"));
  assert!(accepts(regex_to_test, "aa"));
  assert!(accepts(regex_to_test, "aaaa"));
  assert!(accepts(regex_to_test, "abbbbb"));
  assert!(accepts(regex_to_test, "abbba"));
  assert!(!accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "ba"));
  assert!(!accepts(regex_to_test, "babbabb"));

  let regex_to_test = "(a+b)*a";

  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "ba"));
  assert!(accepts(regex_to_test, "aa"));
  assert!(accepts(regex_to_test, "aaaaa"));
  assert!(accepts(regex_to_test, "abbbba"));
  assert!(accepts(regex_to_test, "abbaaba"));
  assert!(!accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "bab"));

  let regex_to_test = "((a)(b))*a";

  assert!(accepts(regex_to_test, "a"));
  assert!(accepts(regex_to_test, "aba"));
  assert!(accepts(regex_to_test, "ababa"));
  assert!(!accepts(regex_to_test, "aaaaa"));
  assert!(!accepts(regex_to_test, "abbbba"));
  assert!(!accepts(regex_to_test, "abbaaba"));
  assert!(!accepts(regex_to_test, "b"));
  assert!(!accepts(regex_to_test, "ab"));
  assert!(!accepts(regex_to_test, "bab"));

  let regex_to_test = "((a+b)c)*";

  assert!(accepts(regex_to_test, ""));
  assert!(accepts(regex_to_test, "ac"));
  assert!(accepts(regex_to_test, "bc"));
  assert!(accepts(regex_to_test, "acbc"));
  assert!(accepts(regex_to_test, "bcac"));
  assert!(!accepts(regex_to_test, "c"));
  assert!(!accepts(regex_to_test, "abc"));
  assert!(!accepts(regex_to_test, "ca"));
  assert!(!accepts(regex_to_test, "cb"));
  assert!(!accepts(regex_to_test, "cab"));

  let regex_to_test = "a*b";

  assert!(accepts(regex_to_test, "b"));
  assert!(accepts(regex_to_test, "ab"));
  assert!(accepts(regex_to_test, "aaaaaaaaaaaaaaaaaaaaaaaaaab"));
  assert!(!accepts(regex_to_test, "c"));
  assert!(!accepts(regex_to_test, "ba"));
  assert!(!accepts(regex_to_test, "aaaaabb"));
  assert!(!accepts(regex_to_test, "cab"));

}

#[test]
fn test_empty_string_and_non_ascii_literals() {
  assert!(accepts("a(ε+b)", "a"));
  assert!(accepts("a(ϵ+b)", "ab"));
  assert!(!accepts("a(ε+b)", "abb"));
  assert!(accepts("ε", ""));
  assert!(!accepts("ε", "a"));

  // Used to panic by slicing a multi-byte character
  assert!(accepts("é*ü", "ééü"));
  assert!(!accepts("é*ü", "ü é"));
  assert!(accepts("(日本)*", "日本日本"));
}

#[test]
fn test_whitespace_is_ignored() {
  assert!(accepts(" a ( b + c ) * ", "abcb"));
}

// A small deterministic random number generator, so the randomized tests are repeatable
struct Random(u64);

impl Random {
  fn below(&mut self, bound: usize) -> usize {
    self.0 ^= self.0 << 13;
    self.0 ^= self.0 >> 7;
    self.0 ^= self.0 << 17;
    (self.0 % bound as u64) as usize
  }
}

// A random regular expression over a and b, written the way a user would type it
fn random_regex(random: &mut Random, depth: usize) -> String {
  match if depth == 0 { random.below(3) } else { random.below(7) } {
    0 => "a".to_owned(),
    1 => "b".to_owned(),
    2 => "ε".to_owned(),
    3 | 4 => format!("{}{}", random_regex(random, depth - 1), random_regex(random, depth - 1)),
    5 => format!("({}+{})", random_regex(random, depth - 1), random_regex(random, depth - 1)),
    _ => format!("({})*", random_regex(random, depth - 1))
  }
}

// An independent way to match a parse tree: the set of positions where a match starting at `start` can end
fn match_ends(token: &Token, characters: &[char], start: usize) -> BTreeSet<usize> {
  let json = serde_json::to_value(token).unwrap();
  match_ends_json(&json, characters, start)
}

fn match_ends_json(token: &serde_json::Value, characters: &[char], start: usize) -> BTreeSet<usize> {
  if let Some(literal) = token.get("Literal").and_then(|literal| literal.as_str()) {
    if literal == EMPTY_STRING {
      return BTreeSet::from([start]);
    }
    let character = literal.chars().next().unwrap();
    return if characters.get(start) == Some(&character) { BTreeSet::from([start + 1]) } else { BTreeSet::new() };
  }
  if let Some(or_operator) = token.get("OrOperator") {
    let mut ends = match_ends_json(&or_operator["left_argument"], characters, start);
    ends.extend(match_ends_json(&or_operator["right_argument"], characters, start));
    return ends;
  }
  if let Some(concatenated) = token.get("ConcatenatedExpression") {
    return match_ends_json(&concatenated["left_argument"], characters, start)
      .into_iter()
      .flat_map(|middle| match_ends_json(&concatenated["right_argument"], characters, middle))
      .collect();
  }
  let inner = &token["KleeneOperator"]["inner_argument"];
  let mut ends = BTreeSet::from([start]);
  let mut frontier = vec![start];
  while let Some(position) = frontier.pop() {
    for end in match_ends_json(inner, characters, position) {
      if ends.insert(end) {
        frontier.push(end);
      }
    }
  }
  ends
}

fn all_strings_up_to(length: usize) -> Vec<String> {
  let mut strings = vec![String::new()];
  let mut previous_length = vec![String::new()];
  for _ in 0..length {
    previous_length = previous_length
      .iter()
      .flat_map(|string| [format!("{string}a"), format!("{string}b")])
      .collect();
    strings.extend(previous_length.iter().cloned());
  }
  strings
}

#[test]
fn test_random_regexes_match_reference_matcher() {
  let mut random = Random(0x9E3779B97F4A7C15);
  let strings = all_strings_up_to(6);

  for _ in 0..300 {
    let regex = random_regex(&mut random, 4);
    let parse_tree = parse_regular_expression(&regex).unwrap();

    for string in &strings {
      let characters: Vec<char> = string.chars().collect();
      let expected = match_ends(&parse_tree, &characters, 0).contains(&characters.len());
      assert_eq!(accepts(&regex, string), expected, "regex {regex:?}, string {string:?}");
    }
  }
}

#[test]
fn test_commands_send_readable_errors() {
  use crate::error::Error;
  use crate::regular_expression_funcs::build_parse_tree;

  assert_eq!(build_parse_tree("a+").unwrap_err(), Error::new("A union (+) needs something after it"));
  assert_eq!(test_string_regex("(a", "a".to_owned()).unwrap_err(), Error::new("A bracket is opened but never closed"));
  assert!(build_parse_tree("a+b").is_ok());
}
