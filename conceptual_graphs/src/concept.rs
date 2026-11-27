use std::fmt;

use link_grammar::english::English;
use link_grammar::{Language, Word};

/// A thing, an action or a property that the sentence mentions, with
/// what the sentence says about it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Concept<'l, L: Language = English> {
    /// The words that name the concept, in sentence order, such as
    /// `has invested`.
    pub words: Vec<Word<'l, L>>,
    pub context: Context<'l, L>,
}

/// What the sentence says about one concept.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Context<'l, L: Language = English> {
    /// Whether the sentence denies the concept, as `eat` in "The girl
    /// doesn't eat pie".
    pub negated: bool,
    /// The words that count the concept, such as `five` in "five pies".
    /// Empty when the sentence doesn't count it.
    pub quantity: Vec<Word<'l, L>>,
}

impl<'l, L: Language> Concept<'l, L> {
    /// A concept of `word` alone, such as a preposition or an adverb.
    pub(crate) fn single(word: Word<'l, L>) -> Self {
        Self {
            words: vec![word],
            context: Context::default(),
        }
    }
}

/// Fuchs's tuple notation for one argument, such as `has-invested`,
/// `pies|five` or `!eat`.
impl<L: Language> fmt::Display for Concept<'_, L> {
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
fn joined<L: Language>(words: &[Word<'_, L>]) -> String {
    words
        .iter()
        .map(|word| word.text())
        .collect::<Vec<_>>()
        .join("-")
}
