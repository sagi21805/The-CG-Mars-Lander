use link_grammar::english::Relation;
use link_grammar::{LinkLabel, LinkRelation, Subscript};

/// A link label that a rule looks for, such as `MVp`. `NIn` is
/// `LinkPattern::at_least(Relation::NI, "n")`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LinkPattern<R: LinkRelation = Relation> {
    relation: R,
    subscript: Subscript,
    matching: Matching,
}

/// How a pattern compares its subscript with a label's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Matching {
    /// The label has every letter of the pattern, and may have more.
    AtLeast,
    /// The label has the pattern's letters and no others.
    Exact,
}

/// The most positions a link-grammar subscript has.
const SUBSCRIPT_POSITIONS: usize = 9;

impl<R: LinkRelation> LinkPattern<R> {
    /// The pattern for `relation` with at least the letters of
    /// `subscript`, which is empty for any subscript.
    pub(crate) const fn at_least(
        relation: R,
        subscript: &'static str,
    ) -> Self {
        Self {
            relation,
            subscript: Subscript::new(subscript),
            matching: Matching::AtLeast,
        }
    }

    /// The pattern for `relation` with the letters of `subscript` and no
    /// others, which is empty for a bare label.
    pub(crate) const fn exact(
        relation: R,
        subscript: &'static str,
    ) -> Self {
        Self {
            matching: Matching::Exact,
            relation,
            subscript: Subscript::new(subscript),
        }
    }

    /// Whether `label` has the pattern's relation and a subscript the
    /// pattern accepts. At least `MVp` matches `MVp` and `MVpn`, and
    /// exactly `MVp` matches only `MVp`. Neither matches `MV` or `MVa`.
    ///
    /// Both are stricter than link-grammar's connector matching, where a
    /// missing letter matches anything, so the bare `E` of "never eats"
    /// doesn't match `Em`.
    pub(crate) fn matches(self, label: LinkLabel<R>) -> bool {
        label.relation == self.relation
            && match self.matching {
                Matching::AtLeast => self.has_letters_in(label.subscript),
                Matching::Exact => self.subscript == label.subscript,
            }
    }

    /// Whether `subscript` has every letter the pattern has, each in the
    /// same position.
    fn has_letters_in(self, subscript: Subscript) -> bool {
        (0..SUBSCRIPT_POSITIONS).all(|position| {
            self.subscript.get(position).is_none_or(|letter| {
                subscript.get(position) == Some(letter)
            })
        })
    }
}
