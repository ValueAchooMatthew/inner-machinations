use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
// Need to add concatonated tokens in future
pub enum Token {
  Literal(String),
  ConcatenatedExpression(Box<(Token, Token)>),
  GroupedExpression(Box<Vec<Token>>),
  OrOperator(Box<OrOperator>),
  KleeneOperator(Box<KleeneOperator>)
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParsingError {
  NoEmptySpaceInParseTree,
  NoneTokenProvided,
  EmptyLeftArg,
  EmptyRightArg,
  NoInnerArg,
  UnableToConcatenate
}

// Consider ditching specific operators in future and instead store the
// 'type of' an operator in a field in the obj
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OrOperator {
  // For now their arguments will exclusively be a single literal
  left_argument: Option<Token>,
  right_argument: Option<Token>
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct KleeneOperator {
  inner_argument: Option<Token>
}

pub trait Operator {
  fn get_operator_character() -> String;
  fn get_operator_name() -> String;
  // Just using insert for now as a test
  fn insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError>;
  fn has_empty_arg(&self) -> bool;
}

pub trait BinaryOperator {
  fn new(left_argument: Option<Token>, right_argument: Option<Token>) -> Self;
  // For now, using Or operator as concrete type in trait since I cannot use any operator using binary operator
  // as it's unsized and thus unsafe
  // Will refactor to work in future
  fn left_insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError>;
  fn right_insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError>;
  fn get_left_argument(&self) -> Option<&Token>;
  fn get_right_argument(&self) -> Option<&Token>; 

}

pub trait UnaryOperator {
  fn new(inner_argument: Option<Token>) -> Self;
  fn get_inner_argument(&self) -> Option<&Token>;
}

impl BinaryOperator for OrOperator {
  fn new(left_argument: Option<Token>, right_argument: Option<Token>) -> Self {
    return OrOperator {
      left_argument,
      right_argument
    }
  }

  // May need to change these impl funcs in future
  // Introduce if let some for non and non none exigent arguments in future
  // Lot's of code duplication, will need to revise later likely with an additional impl statement
  // specifying how to add values to and from left
  fn left_insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {

    if token_to_insert.is_none() {
      return Err(ParsingError::NoneTokenProvided);
    }
    let token_to_insert = token_to_insert.unwrap();

    if let Some(left_side_token) = &mut self.left_argument {
      match left_side_token {
        Token::OrOperator(left_operator) => {
          left_operator.insert_token(Some(token_to_insert))?;
        },
        Token::KleeneOperator(left_operator) => {
          left_operator.insert_token(Some(token_to_insert))?;
        },
        _ => return Err(ParsingError::NoEmptySpaceInParseTree)
      }
      
    } else {
      self.left_argument = Some(token_to_insert);
    }
      
    Ok(())
    
  }
  
  fn right_insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {

    if token_to_insert.is_none() {
      return Err(ParsingError::NoneTokenProvided);
    }
    let token_to_insert = token_to_insert.unwrap();
    
    // First, we have to traverse the right side of the tree from the root to find
    // an or operator token which has a free right argument if not we return a parsing error
    if let Some(right_side_token) = &mut self.right_argument {
      match right_side_token {
        Token::OrOperator(right_operator) => {
          right_operator.insert_token(Some(token_to_insert))?;
        },
        Token::KleeneOperator(right_operator) => {
          right_operator.insert_token(Some(token_to_insert))?;
        },
        _ => {
          return Err(ParsingError::NoEmptySpaceInParseTree);
        }
      }
    } else {
      self.right_argument = Some(token_to_insert);
    }
  
    Ok(())
  }

  fn get_left_argument(&self) -> Option<&Token> {
    return self.left_argument.as_ref();
  }

  fn get_right_argument(&self) -> Option<&Token> {
    return self.right_argument.as_ref();
  }
  
}

impl UnaryOperator for KleeneOperator {
  fn new(inner_argument: Option<Token>) -> Self {
    return KleeneOperator {
      inner_argument
    }
  }

  fn get_inner_argument(&self) -> Option<&Token> {
    return self.inner_argument.as_ref();
  }
}

impl Operator for OrOperator {
  fn get_operator_character() -> String {
    return String::from("+");
  }
  fn get_operator_name() -> String {
    return String::from("Or Operator");
  }

  fn insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {
    // Inserts a token into the first available spot found on either side of the tree
    // Done via BFS to minimize tree depth
    let mut token_queue: Vec<&mut Option<Token>> = vec![];
    token_queue.push(&mut self.left_argument);
    token_queue.push(&mut self.right_argument);
    while !token_queue.is_empty() {
      let current_token = token_queue
        .pop()
        .expect("The queue should have at least one value in it");

      if let Some(current_token) = current_token {
        // Only the or and kleene operator can have children elements so we continue
        // If the current token is neither
        match current_token {
          Token::OrOperator(current_token) => {
            token_queue.insert(0, &mut current_token.left_argument);
            token_queue.insert(0, &mut current_token.right_argument);
          },
          Token::KleeneOperator(current_token) => {
            token_queue.insert(0, &mut current_token.inner_argument);
          },
          _ => continue
        }
      } else {
        // Found spot for token, we dereference the current token
        // and assign it to be the token we want then return out
        *current_token = token_to_insert;
        return Ok(());
      }

    }

    return Err(ParsingError::NoEmptySpaceInParseTree)

  }

  fn has_empty_arg(&self) -> bool {
    return self.left_argument.is_none() || self.right_argument.is_none();
  }

}

impl Operator for KleeneOperator {
  fn get_operator_character() -> String {
    return String::from("*")
  }

  fn get_operator_name() -> String {
    return String::from("Kleene Operator");
  }

  fn insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {
    // Inserts a token into the first available spot found on either side of the tree
    // Done via BFS to minimize tree depth
    if token_to_insert.is_none() {
      return Err(ParsingError::NoneTokenProvided);
    }

    let mut token_queue: Vec<&mut Option<Token>> = vec![];
    token_queue.push(&mut self.inner_argument);
    while !token_queue.is_empty() {
      let current_token = token_queue
        .pop()
        .expect("The queue should have at least one value in it");

      match current_token {
        Some(current_token) => {
          // Only the or and kleene operator can have children elements so we continue
          // If the current token is neither
          match current_token {
            Token::OrOperator(current_token) => {
              token_queue.push(&mut current_token.left_argument);
              token_queue.push(&mut current_token.right_argument);
            },
            Token::KleeneOperator(current_token) => {
              token_queue.push(&mut current_token.inner_argument);
            },
            _ => continue
          }

        },
        None => {
          // Found spot for token, we dereference the current token
          // and assign it to be the token we want then return out
          *current_token = token_to_insert;
          return Ok(());
        }
      }
    }

    return Err(ParsingError::NoEmptySpaceInParseTree)

  }

  fn has_empty_arg(&self) -> bool {
    return self.inner_argument.is_none();
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