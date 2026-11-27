use link_grammar::english::Relation;
use link_grammar::{LinkLabel, LinkRelation, Subscript};

/// A link label that a rule looks for, such as `MVp`. `NIn` is
/// `LinkPattern::new(Relation::NI, "n")`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LinkPattern<R: LinkRelation = Relation> {
    relation: R,
    subscript: Subscript,
}

/// The most positions a link-grammar subscript has.
const SUBSCRIPT_POSITIONS: usize = 9;

impl<R: LinkRelation> LinkPattern<R> {
    /// The pattern for `relation` with at least the letters of
    /// `subscript`, which is empty for any subscript.
    pub(crate) const fn new(relation: R, subscript: &'static str) -> Self {
        Self {
            relation,
            subscript: Subscript::new(subscript),
        }
    }

    /// Whether `label` has the pattern's relation and every subscript
    /// letter the pattern has. `MVp` matches `MVp` and `MVpn`, but not
    /// `MV` or `MVa`.
    ///
    /// This is stricter than link-grammar's connector matching, where a
    /// missing letter matches anything, so the bare `E` of "never eats"
    /// doesn't match `Em`.
    pub(crate) fn matches(self, label: LinkLabel<R>) -> bool {
        label.relation == self.relation
            && (0..SUBSCRIPT_POSITIONS).all(|position| {
                self.subscript.get(position).is_none_or(|letter| {
                    label.subscript.get(position) == Some(letter)
                })
            })
    }
}
