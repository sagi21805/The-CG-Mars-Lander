//! The English concepts that need a walk along links.

use link_grammar::Word;
use link_grammar::english::English;

use super::word_walk::EnglishWordWalk;
use crate::{Concept, Context};

impl<'l> Concept<'l, English> {
    /// The concept of the verb `word`, with its whole verb group.
    pub(crate) fn verb(word: Word<'l>) -> Self {
        Self {
            words: word.verb_group(),
            context: Context::default(),
        }
    }

    /// The concept of the noun `word`, with its proper-noun chain and, as
    /// its quantity, the numbers that count it. Fuchs counts only
    /// objects, and §6.12 item 10 asks for every noun.
    pub(crate) fn noun(word: Word<'l>) -> Self {
        Self {
            words: word.noun_group(),
            context: Context {
                quantity: word.quantifier(),
                ..Context::default()
            },
        }
    }
}
