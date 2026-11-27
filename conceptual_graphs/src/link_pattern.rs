use link_grammar::{LinkLabel, Subscript};

/// A link label that a rule looks for, such as `MVp`. The link type is
/// the label's uppercase part, so `NIn` is `LinkPattern::new("NI", "n")`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LinkPattern {
    link_type: &'static str,
    subscript: Subscript,
}

/// The most positions a link-grammar subscript has.
const SUBSCRIPT_POSITIONS: usize = 9;

impl LinkPattern {
    /// The pattern for `link_type` with at least the letters of
    /// `subscript`, which is empty for any subscript.
    pub(crate) const fn new(
        link_type: &'static str,
        subscript: &'static str,
    ) -> Self {
        Self {
            link_type,
            subscript: Subscript::new(subscript),
        }
    }

    /// Whether `label` has the pattern's link type and every subscript
    /// letter the pattern has. `MVp` matches `MVp` and `MVpn`, but not
    /// `MV` or `MVa`.
    ///
    /// This is stricter than `Subscript::matches`, where a missing letter
    /// matches anything, so the bare `E` of "never eats" doesn't match
    /// `Em`.
    pub(crate) fn matches(self, label: LinkLabel<'_>) -> bool {
        let subscript = label.subscript();
        label.link_type() == self.link_type
            && (0..SUBSCRIPT_POSITIONS).all(|position| {
                self.subscript.get(position).is_none_or(|letter| {
                    subscript.get(position) == Some(letter)
                })
            })
    }
}
