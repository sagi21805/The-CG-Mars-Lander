//! Walks along a linkage's links, in the notation of Fuchs's
//! transformation rules (`theory.pdf`, §5.1, equations 5.1 to 5.4).

use std::iter::successors;

use extend::ext;
use link_grammar::Word;

use crate::link_pattern::LinkPattern;

/// `has` to `invested`. Eq. 5.5.
const PERFECT_TENSE: LinkPattern = LinkPattern::new("PP", "");

/// `would` to `establish`, `doesn't` to `eat`. Eq. 5.7.
const INFINITIVE: LinkPattern = LinkPattern::new("I", "");

/// `Rupert` to `Murdoch`. Eq. 6.2.
const PROPER_NOUN_CHAIN: LinkPattern = LinkPattern::new("G", "");

/// `five` to `pies`. Eq. 5.5.
const NUMBER_DETERMINER: LinkPattern = LinkPattern::new("D", "mcn");

/// `million` to `dollars`. Eq. 5.5 uses `Dmcn` here, and link-grammar
/// 5.13.2 labels this link `NIn`, which splits into `NI` and `n`.
const LARGE_NUMBER_DETERMINER: LinkPattern = LinkPattern::new("NI", "n");

/// `one` to `million`. Eq. 5.5.
const NUMBER_CHAIN: LinkPattern = LinkPattern::new("NN", "");

#[ext(name = WordWalk)]
pub(crate) impl<'l> Word<'l> {
    /// Fuchs's `[word]Δ_L pattern`. The word at the left end of the first
    /// link matching `pattern` whose right end is this word.
    fn left_by(self, pattern: LinkPattern) -> Option<Word<'l>> {
        self.links()
            .find(|link| {
                link.right_word() == self && pattern.matches(link.label())
            })
            .map(|link| link.left_word())
    }

    /// Fuchs's `[word]Δ_R pattern`, the same to the right.
    fn right_by(self, pattern: LinkPattern) -> Option<Word<'l>> {
        self.links()
            .find(|link| {
                link.left_word() == self && pattern.matches(link.label())
            })
            .map(|link| link.right_word())
    }

    /// The verb and its auxiliaries, every word joined to it by `PP` or
    /// `I` links, in sentence order. Eq. 5.5 follows `PP` and eq. 5.7
    /// follows `I`, and both rules must reach one group from either end.
    ///
    /// `invested -> has invested`, `doesn't -> doesn't eat`
    fn verb_group(self) -> Vec<Word<'l>> {
        let first = successors(Some(self), |word| {
            word.left_by(PERFECT_TENSE)
                .or_else(|| word.left_by(INFINITIVE))
        })
        .last()
        .expect("successors starts with the word itself");
        successors(Some(first), |word| {
            word.right_by(PERFECT_TENSE)
                .or_else(|| word.right_by(INFINITIVE))
        })
        .collect()
    }

    /// The noun and the proper nouns chained before it by `G` links.
    ///
    /// `Murdoch -> Rupert Murdoch`
    fn noun_group(self) -> Vec<Word<'l>> {
        let mut group: Vec<_> =
            successors(Some(self), |word| word.left_by(PROPER_NOUN_CHAIN))
                .collect();
        group.reverse();
        group
    }

    /// The number words that count this noun, empty if none. Eq. 5.5
    /// starts from `O.L`, which is the verb, but `Dmcn` joins a number to
    /// a noun, so this starts from the noun.
    ///
    /// `pies -> five`, `dollars -> one million`
    fn quantifier(self) -> Vec<Word<'l>> {
        let first = self
            .left_by(NUMBER_DETERMINER)
            .or_else(|| self.left_by(LARGE_NUMBER_DETERMINER));
        let mut quantifier: Vec<_> =
            successors(first, |word| word.left_by(NUMBER_CHAIN)).collect();
        quantifier.reverse();
        quantifier
    }
}
