//! Walks along a linkage's links, in the notation of Fuchs's
//! transformation rules (`theory.pdf`, §5.1, equations 5.1 to 5.4).

use std::iter::successors;

use extend::ext;
use link_grammar::{Language, Word};

use crate::link_pattern::LinkPattern;

/// Walks from a word along its links, for writing rules.
#[ext(name = WordWalk)]
pub(crate) impl<'l, L: Language> Word<'l, L> {
    /// Fuchs's `[word]Δ_L pattern`. The word at the left end of the first
    /// link matching `pattern` whose right end is this word.
    fn left_by(
        self,
        pattern: LinkPattern<L::Relation>,
    ) -> Option<Word<'l, L>> {
        self.links()
            .find(|link| {
                link.right_word() == self && pattern.matches(link.label())
            })
            .map(|link| link.left_word())
    }

    /// Fuchs's `[word]Δ_R pattern`, the same to the right.
    fn right_by(
        self,
        pattern: LinkPattern<L::Relation>,
    ) -> Option<Word<'l, L>> {
        self.links()
            .find(|link| {
                link.left_word() == self && pattern.matches(link.label())
            })
            .map(|link| link.right_word())
    }

    /// This word and every word reached by repeating [`Self::left_by`]
    /// with any of `patterns`, in sentence order.
    fn left_chain(
        self,
        patterns: &[LinkPattern<L::Relation>],
    ) -> Vec<Word<'l, L>> {
        let mut chain: Vec<_> = successors(Some(self), |word| {
            patterns.iter().find_map(|&pattern| word.left_by(pattern))
        })
        .collect();
        chain.reverse();
        chain
    }

    /// The same to the right.
    fn right_chain(
        self,
        patterns: &[LinkPattern<L::Relation>],
    ) -> Vec<Word<'l, L>> {
        successors(Some(self), |word| {
            patterns.iter().find_map(|&pattern| word.right_by(pattern))
        })
        .collect()
    }
}
