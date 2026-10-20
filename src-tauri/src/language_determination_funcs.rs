use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

use crate::automaton::Automaton;
use crate::models::State;

#[cfg(test)]
mod tests;

// The language is written as a regular expression in the syntax the regular expression builder accepts:
// + for union, * for the Kleene star, ε for the empty string, and ∅ for the empty language
#[tauri::command]
pub fn determine_language_of_automata(
  state_positions: HashMap<String, State>,
  start_state_key: &str
) -> String {

  let (automaton, _) = Automaton::from_state_positions(&state_positions, Some(start_state_key));
  determine_language(&automaton).to_string()

}

// State elimination: add a new initial state with an epsilon transition to the start state, and a new final state
// with an epsilon transition from every accepting state. Then remove the original states one at a time, replacing
// each path through a removed state with a transition labelled by the expression for that path.
// The expression left on the transition from the new initial state to the new final state is the language
pub fn determine_language(automaton: &Automaton) -> RegularExpression {

  let Some(start_state) = automaton.start() else {
    return RegularExpression::EmptyLanguage;
  };

  // States that are unreachable, or can never reach acceptance, contribute nothing
  let mut remaining_states = automaton.useful_states();
  if !remaining_states.contains(&start_state) {
    return RegularExpression::EmptyLanguage;
  }

  let initial_state = automaton.state_count();
  let final_state = initial_state + 1;

  let mut transitions: BTreeMap<(usize, usize), RegularExpression> = BTreeMap::new();
  add_transition(&mut transitions, initial_state, start_state, RegularExpression::EmptyString);

  for &state in &remaining_states {
    for (symbol, target) in automaton.transitions_from(state) {
      if remaining_states.contains(&target) {
        let label = symbol.map_or(RegularExpression::EmptyString, RegularExpression::Literal);
        add_transition(&mut transitions, state, target, label);
      }
    }
    if automaton.is_accepting(state) {
      add_transition(&mut transitions, state, final_state, RegularExpression::EmptyString);
    }
  }

  while let Some(state_to_remove) = choose_state_to_remove(&transitions, &remaining_states) {

    remaining_states.remove(&state_to_remove);

    let self_loop = transitions
      .remove(&(state_to_remove, state_to_remove))
      .map_or(RegularExpression::EmptyString, RegularExpression::star);

    let incoming: Vec<(usize, RegularExpression)> = transitions
      .iter()
      .filter(|((_, to), _)| *to == state_to_remove)
      .map(|((from, _), label)| (*from, label.clone()))
      .collect();

    let outgoing: Vec<(usize, RegularExpression)> = transitions
      .iter()
      .filter(|((from, _), _)| *from == state_to_remove)
      .map(|((_, to), label)| (*to, label.clone()))
      .collect();

    transitions.retain(|(from, to), _| *from != state_to_remove && *to != state_to_remove);

    for (from, incoming_label) in &incoming {
      for (to, outgoing_label) in &outgoing {
        let label = RegularExpression::concatenation([
          incoming_label.clone(),
          self_loop.clone(),
          outgoing_label.clone()
        ]);
        add_transition(&mut transitions, *from, *to, label);
      }
    }
  }

  transitions
    .remove(&(initial_state, final_state))
    .unwrap_or(RegularExpression::EmptyLanguage)

}

fn add_transition(
  transitions: &mut BTreeMap<(usize, usize), RegularExpression>,
  from: usize,
  to: usize,
  label: RegularExpression
) {
  let previous_label = transitions
    .remove(&(from, to))
    .unwrap_or(RegularExpression::EmptyLanguage);

  transitions.insert((from, to), RegularExpression::union([previous_label, label]));
}

// Removes first the state whose removal adds the least to the expressions, which keeps the language short:
// each incoming label is copied once for every outgoing transition beyond the first, and so on.
// Ties go to the lowest numbered state, so the same automaton always gives the same expression
fn choose_state_to_remove(
  transitions: &BTreeMap<(usize, usize), RegularExpression>,
  remaining_states: &BTreeSet<usize>
) -> Option<usize> {

  let length = |label: &RegularExpression| label.to_string().chars().count();

  remaining_states.iter().copied().min_by_key(|&state| {

    let incoming_lengths: Vec<usize> = transitions
      .iter()
      .filter(|((from, to), _)| *to == state && *from != state)
      .map(|(_, label)| length(label))
      .collect();

    let outgoing_lengths: Vec<usize> = transitions
      .iter()
      .filter(|((from, to), _)| *from == state && *to != state)
      .map(|(_, label)| length(label))
      .collect();

    let self_loop_length = transitions.get(&(state, state)).map_or(0, length);
    let path_count = incoming_lengths.len() * outgoing_lengths.len();

    incoming_lengths.iter().sum::<usize>() * outgoing_lengths.len().saturating_sub(1)
      + outgoing_lengths.iter().sum::<usize>() * incoming_lengths.len().saturating_sub(1)
      + self_loop_length * path_count.saturating_sub(1)
  })

}

// A regular expression, kept simplified as it is built so it reads the way a person would write it.
// Concatenation has at least two factors, none of them a concatenation, ε or ∅.
// Union has at least two alternatives, sorted and distinct, none of them a union or ∅
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RegularExpression {
  EmptyLanguage,
  EmptyString,
  Literal(char),
  Concatenation(Vec<RegularExpression>),
  Union(Vec<RegularExpression>),
  Star(Box<RegularExpression>)
}

impl RegularExpression {

  // Whether the expression matches the empty string
  fn is_nullable(&self) -> bool {
    match self {
      Self::EmptyLanguage | Self::Literal(_) => false,
      Self::EmptyString | Self::Star(_) => true,
      Self::Concatenation(factors) => factors.iter().all(Self::is_nullable),
      Self::Union(alternatives) => alternatives.iter().any(Self::is_nullable)
    }
  }

  pub fn concatenation(factors: impl IntoIterator<Item = Self>) -> Self {

    let mut flattened_factors = vec![];

    for factor in factors {
      match factor {
        Self::EmptyLanguage => return Self::EmptyLanguage,
        Self::EmptyString => {},
        Self::Concatenation(inner_factors) => flattened_factors.extend(inner_factors),
        other => flattened_factors.push(other)
      }
    }

    // r*r* is r*
    flattened_factors.dedup_by(|factor, previous_factor| {
      matches!(previous_factor, Self::Star(_)) && factor == previous_factor
    });

    match flattened_factors.len() {
      0 => Self::EmptyString,
      1 => flattened_factors.remove(0),
      _ => Self::Concatenation(flattened_factors)
    }

  }

  pub fn union(alternatives: impl IntoIterator<Item = Self>) -> Self {

    let mut flattened_alternatives = BTreeSet::new();

    for alternative in alternatives {
      match alternative {
        Self::EmptyLanguage => {},
        Self::Union(inner_alternatives) => flattened_alternatives.extend(inner_alternatives),
        other => {
          flattened_alternatives.insert(other);
        }
      }
    }

    if flattened_alternatives.contains(&Self::EmptyString) {

      // ε + rr* and ε + r*r are r*
      let repeated_at_least_once = flattened_alternatives
        .iter()
        .find_map(|alternative| Some((alternative.clone(), alternative.repeated_expression()?)));

      if let Some((alternative, repeated_expression)) = repeated_at_least_once {
        flattened_alternatives.remove(&alternative);
        flattened_alternatives.remove(&Self::EmptyString);
        flattened_alternatives.insert(Self::star(repeated_expression));
        return Self::union(flattened_alternatives);
      }

      // ε adds nothing beside an alternative that matches the empty string already
      let has_other_nullable_alternative = flattened_alternatives
        .iter()
        .any(|alternative| *alternative != Self::EmptyString && alternative.is_nullable());

      if has_other_nullable_alternative {
        flattened_alternatives.remove(&Self::EmptyString);
      }
    }

    // ab + ac is a(b + c), and ac + bc is (a + b)c
    let factored_alternatives = Self::factor_out(flattened_alternatives, Side::Front);
    let mut factored_alternatives = Self::factor_out(factored_alternatives, Side::Back);

    match factored_alternatives.len() {
      0 => Self::EmptyLanguage,
      1 => factored_alternatives.pop_first().expect("The set has one alternative"),
      _ => Self::Union(factored_alternatives.into_iter().collect())
    }

  }

  pub fn star(inner_expression: Self) -> Self {

    match inner_expression {
      Self::EmptyLanguage | Self::EmptyString => Self::EmptyString,
      Self::Star(_) => inner_expression,
      // (ε + r)* is r*, and (r* + s)* is (r + s)*
      Self::Union(alternatives) => {
        let simplified_alternatives = alternatives
          .into_iter()
          .filter(|alternative| *alternative != Self::EmptyString)
          .map(|alternative| match alternative {
            Self::Star(repeated_expression) => *repeated_expression,
            other => other
          });

        match Self::union(simplified_alternatives) {
          union @ Self::Union(_) => Self::Star(Box::new(union)),
          other => Self::star(other)
        }
      },
      // (r*s*)* is (r + s)*, and the same goes for any concatenation of factors that match the empty string
      Self::Concatenation(factors) if factors.iter().all(Self::is_nullable) => Self::star(Self::union(factors)),
      other => Self::Star(Box::new(other))
    }

  }

  // r for an expression of the form rr* or r*r
  fn repeated_expression(&self) -> Option<Self> {

    let Self::Concatenation(factors) = self else {
      return None;
    };

    if let Some(Self::Star(repeated_expression)) = factors.last() {
      if Self::concatenation(factors[..factors.len() - 1].to_vec()) == **repeated_expression {
        return Some(*repeated_expression.clone());
      }
    }

    if let Some(Self::Star(repeated_expression)) = factors.first() {
      if Self::concatenation(factors[1..].to_vec()) == **repeated_expression {
        return Some(*repeated_expression.clone());
      }
    }

    None

  }

  // Groups concatenations that start (or end) with the same factor and factors it out
  fn factor_out(alternatives: BTreeSet<Self>, side: Side) -> BTreeSet<Self> {

    let mut factored_alternatives = BTreeSet::new();
    let mut remainders_by_common_factor: BTreeMap<Self, Vec<Self>> = BTreeMap::new();

    for alternative in alternatives {
      match alternative {
        Self::Concatenation(mut factors) => {
          let common_factor = match side {
            Side::Front => factors.remove(0),
            Side::Back => factors.pop().expect("A concatenation has at least two factors")
          };
          remainders_by_common_factor
            .entry(common_factor)
            .or_default()
            .push(Self::concatenation(factors));
        },
        other => {
          factored_alternatives.insert(other);
        }
      }
    }

    for (common_factor, mut remainders) in remainders_by_common_factor {

      let remainder = if remainders.len() == 1 {
        remainders.remove(0)
      } else {
        Self::union(remainders)
      };

      let factors = match side {
        Side::Front => [common_factor, remainder],
        Side::Back => [remainder, common_factor]
      };
      factored_alternatives.insert(Self::concatenation(factors));
    }

    factored_alternatives

  }

}

enum Side {
  Front,
  Back
}

impl fmt::Display for RegularExpression {

  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::EmptyLanguage => write!(formatter, "∅"),
      Self::EmptyString => write!(formatter, "ε"),
      Self::Literal(character) => write!(formatter, "{character}"),
      Self::Concatenation(factors) => {
        for factor in factors {
          match factor {
            Self::Union(_) => write!(formatter, "({factor})")?,
            _ => write!(formatter, "{factor}")?
          }
        }
        Ok(())
      },
      Self::Union(alternatives) => {
        for (index, alternative) in alternatives.iter().enumerate() {
          if index > 0 {
            write!(formatter, " + ")?;
          }
          write!(formatter, "{alternative}")?;
        }
        Ok(())
      },
      Self::Star(repeated_expression) => match **repeated_expression {
        Self::Literal(_) => write!(formatter, "{repeated_expression}*"),
        _ => write!(formatter, "({repeated_expression})*")
      }
    }
  }

}
