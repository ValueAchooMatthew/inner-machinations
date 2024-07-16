use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Token {
  BinaryOperator(char),
  UnaryOperator(char),
  GroupedExpression(Box<(Token, Token)>),
  Literal(String)
}
#[derive(Clone)]

pub enum ParseTree {
  Or(OrOperator),
  Kleene(KleeneOperator),
}
#[derive(Debug)]
pub enum ParsingError {
  NoLeftArg,
  NoRightArg,
  NoInnerArg
}

#[derive(Clone)]
pub struct OrOperator {
  // For now their arguments will exclusively be a single literal
  left_argument: Token,
  right_argument: Token
}

#[derive(Clone)]
pub struct KleeneOperator {
  inner_argument: Token
}

pub trait Operator {
  fn get_operator_character() -> String;
  fn get_operator_name() -> String;
}

pub trait BinaryOperator {
  fn new(left_argument: Token, right_argument: Token) -> Self;
}

pub trait UnaryOperator {
  fn new(inner_argument: Token) -> Self;
}

impl BinaryOperator for OrOperator {
  fn new(left_argument: Token, right_argument: Token) -> Self {
    return OrOperator {
      left_argument,
      right_argument,
    }
  }
}

impl Into<Token> for OrOperator {
  fn into(self) -> Token {
    let grouped_expression = vec![self.left_argument, self.right_argument];
    return Token::GroupedExpression(Box::new((
      grouped_expression.into(), 
      Token::BinaryOperator('+'))
    ));

  }
}

impl Into<Token> for ParseTree {
  fn into(self) -> Token {
    match self {
      ParseTree::Or(op) => {
        return op.into();
      },
      ParseTree::Kleene(op) => {
        return op.into()
      }

    }
  }
}


impl Into<Token> for KleeneOperator {
  fn into(self) -> Token {
    return Token::GroupedExpression(Box::new((
      self.inner_argument, 
      Token::UnaryOperator('*'))
    ))
  }
}

impl UnaryOperator for KleeneOperator {
  fn new(inner_argument: Token) -> Self {
    return KleeneOperator {
      inner_argument
    }
  }
}

impl Operator for OrOperator {
  fn get_operator_character() -> String {
    return String::from("+");
  }
  fn get_operator_name() -> String {
      return String::from("Or Operator");
  }
}

impl Operator for KleeneOperator {
  fn get_operator_character() -> String {
    return String::from("*")
  }

  fn get_operator_name() -> String {
    return String::from("Kleene Operator");
  }
}

impl Into<Token> for &str {

  fn into(self) -> Token {
    let mut value = String::new();

    let forbidden_characters = HashSet::from(['(', ')', '[', ']', '*', '+']);

    for c in self.chars() {
      if !c.is_whitespace() && !forbidden_characters.contains(&c) {
        value.push(c);
      } else {
        return Token::Literal(value);
      }

    }
    return Token::Literal(value)
  }
}

impl Into<Token> for Vec<Token> {
  fn into(mut self) -> Token {
    // The Vec must be of at least length 2 for this to work, I will
    // refactor to handle base cases later
    if self.len() == 1 {
      return self.get(0)
        .expect("The vector should have exactly one element")
        .to_owned();
    }

    let mut collected_tokens = Token::GroupedExpression(Box::new((
      self.get(0)
        .expect("The vector should have a least two entries")
        .to_owned(),
      self.get(1)
        .expect("The vector should have a least two entries")
        .to_owned(),
    )));

    let updated_vec = self.split_off(2);

    for token in updated_vec.clone() {
      collected_tokens = Token::GroupedExpression(Box::new((collected_tokens, token)));
    };

    return collected_tokens;

  }
}

