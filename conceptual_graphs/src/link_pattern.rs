use link_grammar::{LinkLabel, Subscript};

/// A link label that a rule looks for, such as `MVp`. The link type is
/// the label's uppercase part, so `NIn` is `LinkPattern::new("NI", "n")`.
///
/// ```
/// use conceptual_graphs::LinkPattern;
/// use link_grammar::{Dictionary, Language, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence = Sentence::new(&dictionary, "The girl never eats pie.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let labels: Vec<_> = linkage.links().map(|link| link.label()).collect();
///
/// let object = LinkPattern::new("O", "");
/// let before_verb = LinkPattern::new("E", "m");
/// assert!(labels.iter().any(|&label| object.matches(label)));
/// // "never" links to "eats" with a bare `E`, which lacks the `m`.
/// assert!(!labels.iter().any(|&label| before_verb.matches(label)));
/// # Ok::<(), link_grammar::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkPattern {
    link_type: &'static str,
    subscript: Subscript,
}

/// The most positions a link-grammar subscript has.
const SUBSCRIPT_POSITIONS: usize = 9;

impl LinkPattern {
    /// The pattern for `link_type` with at least the letters of
    /// `subscript`, which is empty for any subscript.
    pub const fn new(
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
    pub fn matches(self, label: LinkLabel<'_>) -> bool {
        let subscript = label.subscript();
        label.link_type() == self.link_type
            && (0..SUBSCRIPT_POSITIONS).all(|position| {
                self.subscript.get(position).is_none_or(|letter| {
                    subscript.get(position) == Some(letter)
                })
            })
    }
}
