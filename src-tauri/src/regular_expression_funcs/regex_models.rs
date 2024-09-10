use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
// Need to add concatonated tokens in future
pub enum Token {
  Literal(String),
  ConcatenatedExpression(Box<ConcatenatedExpression>),
  GroupedExpression(Box<Vec<Token>>),
  OrOperator(Box<OrOperator>),
  KleeneOperator(Box<KleeneOperator>)
}

pub type TokenArray = Vec<Token>;

pub trait TokenArrayMethods {
  fn does_contain_grouped_expression(&self) -> Option<(Token, usize)>;
  fn does_contain_kleene_token(&self) -> Option<(Token, usize)>;
  fn concatenate_tokens(&self) -> Token;
}

impl TokenArrayMethods for TokenArray {
  fn does_contain_grouped_expression(&self) -> Option<(Token, usize)> {
    for (index, token) in self.into_iter().enumerate() {
      match token {
        Token::GroupedExpression(_) => return Some((token.to_owned(), index)),
        _ => continue
      }
    }
    return None;
  }
  fn does_contain_kleene_token(&self) -> Option<(Token, usize)> {
    for (index, token) in self.into_iter().enumerate() {
      match token {
        Token::KleeneOperator(kleene_operator) => {
          // If a kleene operator is already filled we continue
          if kleene_operator.has_empty_arg() {
            return Some((token.to_owned(), index))
          }
        },
        _ => continue
      }
    }
    return None;
  }

  fn concatenate_tokens(&self) -> Token {
    if self.len() == 1 {
       return self
         .get(0)
         .expect("The array should have at least a single element")
         .to_owned();
   
     } else if self.len() == 2 {
   
       let first_token = self
         .get(0)
         .expect("The array should have at least 2 elements");
   
       let second_token = self
         .get(1)
         .expect("The array should have at least 2 elements");
       return Token::ConcatenatedExpression(Box::new(ConcatenatedExpression::new(
         Some(first_token.to_owned()), 
         Some(second_token.to_owned())
       )));
     }
   
     let midpoint = self.len().div_ceil(2);
     let first_half_of_self = &self[..midpoint].to_vec();
     let second_half_of_self = &self[midpoint..].to_vec();
   
     return Token::ConcatenatedExpression(Box::new(ConcatenatedExpression::new(
       Some(Self::concatenate_tokens(first_half_of_self)), 
       Some(Self::concatenate_tokens(second_half_of_self))
     )));
   
   }
}


#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParsingError {
  NoEmptySpaceInParseTree,
  NoneTokenProvided,
  EmptyLeftArg,
  EmptyRightArg,
  NoInnerArg,
  UnableToConcatenate,
  MissingClosingBracket
}

// Consider ditching specific operators in future and instead store the
// 'type of' an operator in a field in the obj
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OrOperator {
  operator_character: String,
  operator_name: String,
  left_argument: Option<Token>,
  right_argument: Option<Token>
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct KleeneOperator {
  operator_character: String,
  operator_name: String,
  inner_argument: Option<Token>
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct ConcatenatedExpression {
  operator_character: String,
  operator_name: String,
  left_argument: Option<Token>,
  right_argument: Option<Token>
}

pub trait Operator {
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

impl Operator for ConcatenatedExpression {
  fn insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {
    if !self.has_empty_arg() {
      return Err(ParsingError::NoEmptySpaceInParseTree)
    }
    if self.get_left_argument().is_none() {
      self.left_insert_token(token_to_insert)?;
    } else {
      self.right_insert_token(token_to_insert)?;
    }
    Ok(())
  }
  fn has_empty_arg(&self) -> bool {
    return self.left_argument.is_none() || self.right_argument.is_none();
  }
}

impl BinaryOperator for ConcatenatedExpression {
  fn new(left_argument: Option<Token>, right_argument: Option<Token>) -> Self {
    return ConcatenatedExpression {
      operator_character: String::from("⋅"),
      operator_name: String::from("Concatenated Expressions"),
      left_argument,
      right_argument
    }
  }
  fn left_insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {
    if token_to_insert.is_none() {
      return Err(ParsingError::NoneTokenProvided);
    }
    self.left_argument = token_to_insert;
    Ok(())
  }
  fn right_insert_token(&mut self, token_to_insert: Option<Token>) -> Result<(), ParsingError> {
    if token_to_insert.is_none() {
      return Err(ParsingError::NoneTokenProvided);
    }
    self.right_argument = token_to_insert;
    Ok(())
  }
  fn get_left_argument(&self) -> Option<&Token> {
    return self.left_argument.as_ref();
  }
  fn get_right_argument(&self) -> Option<&Token> {
    return self.right_argument.as_ref();
  }
}


impl BinaryOperator for OrOperator {
  fn new(left_argument: Option<Token>, right_argument: Option<Token>) -> Self {
    return OrOperator {
      operator_character: String::from("+"),
      operator_name: String::from("Or"),
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
      operator_character: String::from("*"),
      operator_name: String::from("Kleene"),
      inner_argument
    }
  }

  fn get_inner_argument(&self) -> Option<&Token> {
    return self.inner_argument.as_ref();
  }
}

impl Operator for OrOperator {
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

// Potentially change in future for cleaner documentation
impl Token {
  // This method is intended to work exclusively for strings in which not whitespace or demarkated tokens
  // Are placed beside each other to generate either a single token literal or a tree of concatenated token
  // Literals for further use in the parsing step
  pub fn parse_string_to_tokens(stream: &str) -> (Self, usize) {
    let forbidden_characters = HashSet::from([' ', '(', ')',  '+', '*']);
    let mut characters_to_skip = 0;

    let mut literals_encountered: TokenArray = Vec::new();

    for c in stream.chars() {
      if !forbidden_characters.contains(&c) {
        literals_encountered.push(Token::Literal(c.to_string()));
        characters_to_skip += 1;
      } else {
        break;
      }
    };

    let parsed_token = literals_encountered.concatenate_tokens();

    return (parsed_token, characters_to_skip);
  }

}