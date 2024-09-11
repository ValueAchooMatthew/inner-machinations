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
  fn does_contain_unfilled_kleene_token(&self) -> Option<(Token, usize)>;
  fn get_middlemost_or_token(&self) -> Option<(Token, usize)>;
  fn concatenate_tokens(&self) -> Token;
  fn can_continue_parsing(&self) -> bool;
  fn parse_tokens(self) -> Result<Token, ParsingError>;
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

  fn does_contain_unfilled_kleene_token(&self) -> Option<(Token, usize)> {
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
  
  // Gets the or token closest to the middle of array so more optimally packed tree can be created

  fn get_middlemost_or_token(&self) -> Option<(Token, usize)> {

    let middle_of_token_array = self.len() / 2;
    let mut middlemost_or_token = None;
    let mut distance_of_closest_or_operator = usize::MAX;

    for (index, token) in self.into_iter().enumerate() {
      match token {
        Token::OrOperator(or_operator) => {
          
          if !or_operator.has_empty_arg() {
            continue;
          }

          let current_distance_from_middle = middle_of_token_array.abs_diff(index);
          if let Some(_) = middlemost_or_token {
            if current_distance_from_middle < distance_of_closest_or_operator {
              middlemost_or_token = Some((token.to_owned(), index));
              distance_of_closest_or_operator = current_distance_from_middle;
            }

          } else {
            middlemost_or_token = Some((token.to_owned(), index));
            distance_of_closest_or_operator = current_distance_from_middle;
          }
            
        },
        _ => continue
      }
    }
    return middlemost_or_token;
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
   
     let midpoint = self.len()/2;
     let first_half_of_self = &self[..midpoint].to_vec();
     let second_half_of_self = &self[midpoint..].to_vec();
   
     return Token::ConcatenatedExpression(Box::new(ConcatenatedExpression::new(
       Some(Self::concatenate_tokens(first_half_of_self)), 
       Some(Self::concatenate_tokens(second_half_of_self))
     )));
   
   }

   fn can_continue_parsing(&self) -> bool {
    for token in self {
      match token {
        Token::OrOperator(or_operator) => {
          if or_operator.has_empty_arg() {
            return true;
          }
        },
        Token::KleeneOperator(kleene_operator) => {
          if kleene_operator.has_empty_arg() {
            return true;
          }
        },
        // Done so grouped expressions are continued to be broked up in the parsing step
        Token::GroupedExpression(_) => {
          return true;
        },
        _=> continue
      }
    }
    return false;
  }
  
  fn parse_tokens(mut self) -> Result<Token, ParsingError> {
    // Must give priority to grouped expressions
    // Parse grouped expressions first?
    if self.len() == 0 {
      return Err(ParsingError::NoInnerArg)
    } else if self.len() == 1 {
      match self.get(0).expect("The array should have at least a single element") {
        // If it's a grouped expression, do nothing and continue breaking it apart
        Token::GroupedExpression(_) => (),
        _ => {
          return Ok(self.concatenate_tokens())
        }
      }
    } else if !self.can_continue_parsing() {
      return Ok(self.concatenate_tokens());
    }

    // parsing all regular expressions into their proper form FIRST prior to any operations
    let mut has_grouped_expression = self.does_contain_grouped_expression();
    while has_grouped_expression.is_some() {
      let (grouped_expression, index) = has_grouped_expression.unwrap();
      self
        .remove(index);
      match grouped_expression {
        Token::GroupedExpression(grouped_expression) => {
          let parsed_grouped_expression = Self::parse_tokens(*grouped_expression)?;
          self.insert(index, parsed_grouped_expression);
        },
        _ => panic!("The supplied token should be a grouped expression!")
      }
      has_grouped_expression = self.does_contain_grouped_expression();
    };

    let mut has_kleene_token = self.does_contain_unfilled_kleene_token();
    while has_kleene_token.is_some() {
      let (kleene_token, index) = has_kleene_token.unwrap();
      match kleene_token {
        Token::KleeneOperator(mut kleene_operator) => {
          let left_token = self
            .get(index.checked_sub(1).ok_or_else( || {ParsingError::NoneTokenProvided})?)
            .cloned();

          kleene_operator
            .insert_token(left_token)?;

          self.drain(index-1..=index);
          self.insert(index-1, Token::KleeneOperator(kleene_operator));
          },
          _ => panic!("The supplied token should be a kleene token!")
      }
      has_kleene_token = self.does_contain_unfilled_kleene_token();
    };

    // We use this as a catch all that will recursively parse the arguments to the or_operator when encountered
    if let Some((mut or_token, index)) = self.get_middlemost_or_token() {
      match &mut or_token {
        Token::OrOperator(or_operator) => {

          let tokens_to_left = self.get(..index).ok_or_else(|| {
            ParsingError::EmptyLeftArg
          })?.to_owned();
          
          // Ugly fix but it's possible for tokens to left or right to return Some even when the vec size is 0,
          // which i do not want, hence the manual checking. Todo: Look for cleaner fix in future
          if tokens_to_left.len() == 0 {
            return Err(ParsingError::EmptyLeftArg);
          }

          let tokens_to_right = self.get(index + 1..).ok_or_else(|| {
            ParsingError::EmptyRightArg
          })?.to_owned();

          if tokens_to_right.len() == 0 {
            return Err(ParsingError::EmptyRightArg);
          }

          let parsed_left_tokens = Self::parse_tokens(tokens_to_left)?;
          let parsed_right_tokens = Self::parse_tokens(tokens_to_right)?;

          or_operator.left_insert_token(Some(parsed_left_tokens))?;
          or_operator.right_insert_token(Some(parsed_right_tokens))?;

          return Ok(or_token);

        },
        _ => panic!("The supplied token should be an or token!")

      }

    }

    return Self::parse_tokens(self);

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

    let mut literals_encountered: TokenArray = Vec::new();

    for c in stream.chars() {
      if !forbidden_characters.contains(&c) {
        literals_encountered.push(Token::Literal(c.to_string()));
      } else {
        break;
      }
    };

    let parsed_token = literals_encountered.concatenate_tokens();

    return (parsed_token, literals_encountered.len());
  }

  pub fn verify_syntactic_correctness(&self) -> Result<(), ParsingError> {
    // If we come across an operator which has an None argument, a value was not
    // properly supplied to the operator and thus the tree is syntactically incorrect
    // We check for this using DFS
  
    match self {
      Token::KleeneOperator(operator) => { 
        if operator.has_empty_arg() {
          Err(ParsingError::NoInnerArg)
        } else {
          let inner_argument = operator.get_inner_argument().unwrap();
          Self::verify_syntactic_correctness(inner_argument)
        }
      },
      Token::OrOperator(operator) => {
        if operator.has_empty_arg() {
          if operator.get_left_argument().is_none() {
            Err(ParsingError::EmptyLeftArg)
          } else {
            Err(ParsingError::EmptyRightArg)
          }
        } else {
          Self::verify_syntactic_correctness(
            operator.get_left_argument().unwrap()
          )?;
          Self::verify_syntactic_correctness(
            operator.get_right_argument().unwrap()
          )?;
          Ok(())
        }
      },
      _ => Ok(())
    }
  }

}