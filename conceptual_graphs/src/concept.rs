use std::fmt;

use link_grammar::Word;

use crate::word_walk::WordWalk;

/// A thing, an action or a property that the sentence mentions, with
/// what the sentence says about it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Concept<'l> {
    /// The words that name the concept, in sentence order, such as
    /// `has invested`.
    pub words: Vec<Word<'l>>,
    pub context: Context<'l>,
}

/// What the sentence says about one concept.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Context<'l> {
    /// Whether the sentence denies the concept, as `eat` in "The girl
    /// doesn't eat pie".
    pub negated: bool,
    /// The words that count the concept, such as `five` in "five pies".
    /// Empty when the sentence doesn't count it.
    pub quantity: Vec<Word<'l>>,
}

impl<'l> Concept<'l> {
    /// The concept of the verb `word`, with its whole verb group.
    pub fn verb(word: Word<'l>) -> Self {
        Self {
            words: word.verb_group(),
            context: Context::default(),
        }
    }

    /// The concept of the noun `word`, with its proper-noun chain and, as
    /// its quantity, the numbers that count it. Fuchs counts only
    /// objects, and §6.12 item 10 asks for every noun.
    pub fn noun(word: Word<'l>) -> Self {
        Self {
            words: word.noun_group(),
            context: Context {
                quantity: word.quantifier(),
                ..Context::default()
            },
        }
    }

    /// A concept of `word` alone, such as a preposition or an adverb.
    pub fn single(word: Word<'l>) -> Self {
        Self {
            words: vec![word],
            context: Context::default(),
        }
    }
}

/// Fuchs's tuple notation for one argument, such as `has-invested`,
/// `pies|five` or `!eat`.
impl fmt::Display for Concept<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.context.negated {
            f.write_str("!")?;
        }
        f.write_str(&joined(&self.words))?;
        if !self.context.quantity.is_empty() {
            write!(f, "|{}", joined(&self.context.quantity))?;
        }
        Ok(())
    }
}

/// The words' text joined with `-`, as in `one-million`.
fn joined(words: &[Word<'_>]) -> String {
    words
        .iter()
        .map(|word| word.text())
        .collect::<Vec<_>>()
        .join("-")
}
