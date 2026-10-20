use serde::{Deserialize, Serialize};

// The parse tree is sent to the frontend, which draws it, so the serialized names here are shared with
// the Token types in src/lib/types
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
pub enum Token {
  Literal(String),
  ConcatenatedExpression(Box<ConcatenatedExpression>),
  OrOperator(Box<OrOperator>),
  KleeneOperator(Box<KleeneOperator>)
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParsingError {
  // Nothing to parse, as in "" or "  "
  EmptyExpression,
  // As in "()"
  EmptyBrackets,
  // A union with nothing before it, as in "+a"
  EmptyLeftArg,
  // A union with nothing after it, as in "a+"
  EmptyRightArg,
  // A Kleene star with nothing before it, as in "*a" or "a+*"
  NoInnerArg,
  // As in "(a"
  MissingClosingBracket,
  // As in "a)"
  UnmatchedClosingBracket
}

// What the user sees on the regular expression page
impl std::fmt::Display for ParsingError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    let message = match self {
      ParsingError::EmptyExpression => "Type a regular expression",
      ParsingError::EmptyBrackets => "Brackets need something inside them",
      ParsingError::EmptyLeftArg => "A union (+) needs something before it",
      ParsingError::EmptyRightArg => "A union (+) needs something after it",
      ParsingError::NoInnerArg => "A star (*) needs something before it to repeat",
      ParsingError::MissingClosingBracket => "A bracket is opened but never closed",
      ParsingError::UnmatchedClosingBracket => "A bracket is closed but was never opened"
    };
    write!(formatter, "{message}")
  }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OrOperator {
  operator_character: String,
  operator_name: String,
  left_argument: Token,
  right_argument: Token
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct KleeneOperator {
  operator_character: String,
  operator_name: String,
  inner_argument: Token
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct ConcatenatedExpression {
  operator_character: String,
  operator_name: String,
  left_argument: Token,
  right_argument: Token
}

impl OrOperator {
  pub fn new(left_argument: Token, right_argument: Token) -> Self {
    return OrOperator {
      operator_character: String::from("+"),
      operator_name: String::from("Or"),
      left_argument,
      right_argument
    }
  }

  pub fn get_left_argument(&self) -> &Token {
    return &self.left_argument;
  }

  pub fn get_right_argument(&self) -> &Token {
    return &self.right_argument;
  }
}

impl KleeneOperator {
  pub fn new(inner_argument: Token) -> Self {
    return KleeneOperator {
      operator_character: String::from("*"),
      operator_name: String::from("Kleene"),
      inner_argument
    }
  }

  pub fn get_inner_argument(&self) -> &Token {
    return &self.inner_argument;
  }
}

impl ConcatenatedExpression {
  pub fn new(left_argument: Token, right_argument: Token) -> Self {
    return ConcatenatedExpression {
      operator_character: String::from("⋅"),
      operator_name: String::from("Concatenated Expressions"),
      left_argument,
      right_argument
    }
  }

  pub fn get_left_argument(&self) -> &Token {
    return &self.left_argument;
  }

  pub fn get_right_argument(&self) -> &Token {
    return &self.right_argument;
  }
}
