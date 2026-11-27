//! The walks that gather an English word group (`theory.pdf`, §5.1,
//! equations 5.5 and 5.7).

use std::iter::successors;

use extend::ext;
use link_grammar::Word;
use link_grammar::english::{English, Relation};

use crate::link_pattern::LinkPattern;
use crate::word_walk::WordWalk;

/// `has` to `invested`. Eq. 5.5.
const PERFECT_TENSE: LinkPattern = LinkPattern::new(Relation::PP, "");

/// `would` to `establish`, `doesn't` to `eat`. Eq. 5.7.
const INFINITIVE: LinkPattern = LinkPattern::new(Relation::I, "");

/// `Rupert` to `Murdoch`. Eq. 6.2.
const PROPER_NOUN_CHAIN: LinkPattern = LinkPattern::new(Relation::G, "");

/// `five` to `pies`. Eq. 5.5.
const NUMBER_DETERMINER: LinkPattern =
    LinkPattern::new(Relation::D, "mcn");

/// `million` to `dollars`. Eq. 5.5 uses `Dmcn` here, and link-grammar
/// 5.13.2 labels this link `NIn`, which is `NI` with the subscript `n`.
const LARGE_NUMBER_DETERMINER: LinkPattern =
    LinkPattern::new(Relation::NI, "n");

/// `one` to `million`. Eq. 5.5.
const NUMBER_CHAIN: LinkPattern = LinkPattern::new(Relation::NN, "");

#[ext(name = EnglishWordWalk)]
pub(crate) impl<'l> Word<'l, English> {
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
