//! Fuchs's transformation rules (`theory.pdf`, §4.4, §5.1 and §6.3). Each
//! rule works like a regex. It checks whether one link has the shape it
//! looks for, and returns the relation it found, or `None`.

use link_grammar::Link;
use link_grammar::english::Relation;

use crate::language_rules::FoundRelation;
use crate::link_pattern::{AnyPattern, LinkPattern};
use crate::word_walk::WordWalk;
use crate::{Concept, ConceptualRelation};

/// A singular or plural subject, `Ss` or `Sp`.
const SUBJECT: [LinkPattern; 2] = [
    LinkPattern::at_least(Relation::S, "s"),
    LinkPattern::at_least(Relation::S, "p"),
];
const RELATIVE_SUBJECT: LinkPattern =
    LinkPattern::at_least(Relation::B, "s");
const OBJECT: LinkPattern = LinkPattern::at_least(Relation::O, "");
/// §4.4 writes `MV`, and link-grammar 5.13.2 labels "eats fast" `MVa`. A
/// bare `MV` would also catch `MVp`, which is [`preposition`]'s.
const ADVERB_AFTER_VERB: LinkPattern =
    LinkPattern::at_least(Relation::MV, "a");
const ADVERB_BEFORE_VERB: LinkPattern =
    LinkPattern::at_least(Relation::E, "m");
const PREPOSITION: LinkPattern = LinkPattern::at_least(Relation::MV, "p");
/// Eq. 5.6 writes `Jp`, and Appendix C defines `J` as the link from any
/// preposition to its object.
const PREPOSITION_OBJECT: LinkPattern =
    LinkPattern::at_least(Relation::J, "");

/// `girl -S- eats` gives `Agent(girl, eats)`. §4.4 step 2, eq. 5.5,
/// eq. 6.2.
pub(crate) fn subject<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    SUBJECT
        .matches_any(link.label())
        .then(|| ConceptualRelation::Agent {
            actor: Concept::noun(link.left_word()),
            action: Concept::verb(link.right_word()),
        })
}

/// `business -Bs- would establish` gives `Agent(business,
/// would-establish)`. Eq. 5.7. The clause's object comes from
/// [`object`], because its verb has its own `O` link.
pub(crate) fn relative_subject<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    RELATIVE_SUBJECT.matches(link.label()).then(|| {
        ConceptualRelation::Agent {
            actor: Concept::noun(link.left_word()),
            action: Concept::verb(link.right_word()),
        }
    })
}

/// `eats -O- pie` gives `Object(eats, pie)`. §4.4 step 3, eq. 5.5.
pub(crate) fn object<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    OBJECT
        .matches(link.label())
        .then(|| ConceptualRelation::Object {
            action: Concept::verb(link.left_word()),
            object: Concept::noun(link.right_word()),
        })
}

/// `eats -MVa- fast` gives `Manner(eats, fast)`. §4.4 step 4.
pub(crate) fn adverb_after_verb<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    ADVERB_AFTER_VERB.matches(link.label()).then(|| {
        ConceptualRelation::Manner {
            action: Concept::verb(link.left_word()),
            manner: Concept::single(link.right_word()),
        }
    })
}

/// `privately -Em- managed` gives `Manner(managed, privately)`. Eq. 6.7.
pub(crate) fn adverb_before_verb<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    ADVERB_BEFORE_VERB.matches(link.label()).then(|| {
        ConceptualRelation::Manner {
            action: Concept::verb(link.right_word()),
            manner: Concept::single(link.left_word()),
        }
    })
}

/// `invested -MVp- in -J- Hungary` gives `Prepositional(invested, in,
/// Hungary)`. Eq. 5.6.
pub(crate) fn preposition<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    if !PREPOSITION.matches(link.label()) {
        return None;
    }
    let object = link.right_word().right_by(PREPOSITION_OBJECT)?;
    Some(ConceptualRelation::Prepositional {
        head: Concept::verb(link.left_word()),
        preposition: Concept::single(link.right_word()),
        object: Concept::noun(object),
    })
}
