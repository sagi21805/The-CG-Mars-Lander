//! Fuchs's transformation rules (`theory.pdf`, §4.4, §5.1 and §6.2 to
//! §6.4). Each rule works like a regex. It checks whether one link has the
//! shape it looks for, and returns the relation it found, or `None`.

use link_grammar::english::{English, Relation};
use link_grammar::{Link, Word};

use super::word_walk::EnglishWordWalk;
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
/// `Hungary` to `'s`. Eq. 5.8 writes `YS`, and `YP` is the same link
/// after a plural, as in "the boys' toys".
const POSSESSOR: [LinkPattern; 2] = [
    LinkPattern::at_least(Relation::YS, ""),
    LinkPattern::at_least(Relation::YP, ""),
];
/// `'s` to `business`. Eq. 5.8.
const DETERMINER: LinkPattern = LinkPattern::at_least(Relation::D, "");
/// `invested` to `to`. Eq. 6.1.
const PURPOSE: LinkPattern = LinkPattern::at_least(Relation::MV, "i");
/// `ceased` to `to`. Eq. 6.3.
const TO: LinkPattern = LinkPattern::at_least(Relation::TO, "");
/// `to` to `exist`. Eq. 6.1 and eq. 6.3.
const TO_INFINITIVE: LinkPattern = LinkPattern::at_least(Relation::I, "");
/// `began` to `singing`. Eq. 6.4.
const PRESENT_PARTICIPLE: LinkPattern =
    LinkPattern::at_least(Relation::P, "g");
/// `big` to `dog`. Eq. 6.5.
const ADJECTIVE: LinkPattern = LinkPattern::at_least(Relation::A, "");
/// `stock` to `market`. Eq. 6.6.
const NOUN_MODIFIER: LinkPattern = LinkPattern::at_least(Relation::AN, "");
/// The object of a form of "to be". §6.4 writes `Ost` and `Opt`, and
/// link-grammar 5.13.2 labels these `Osm`, `Oum` and `Opt`.
const BE_OBJECT: [LinkPattern; 4] = [
    LinkPattern::at_least(Relation::O, "st"),
    LinkPattern::at_least(Relation::O, "pt"),
    LinkPattern::at_least(Relation::O, "sm"),
    LinkPattern::at_least(Relation::O, "um"),
];
/// `owner` to `of`. Eq. 6.9 writes `Mp`, and link-grammar 5.13.2 labels
/// an "of" phrase `Mf`.
const NOUN_PREPOSITION: [LinkPattern; 2] = [
    LinkPattern::at_least(Relation::M, "p"),
    LinkPattern::at_least(Relation::M, "f"),
];

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

/// `eats -O- pie`, where `girl -S- eats`, gives `Svo(girl, eats, pie)`.
/// §4.4 step 6, eq. 5.5, eq. 6.2, eq. 6.8. The subject may link to any
/// word of the verb group, as in "the fund has invested money".
pub(crate) fn svo<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    if !OBJECT.matches(link.label()) {
        return None;
    }
    let subject = subject_of(link.left_word())?;
    Some(ConceptualRelation::Svo {
        subject: Concept::noun(subject),
        verb: Concept::verb(link.left_word()),
        object: Concept::noun(link.right_word()),
    })
}

/// `business -Bs- would establish -O- ATMs` gives `SubSvo(business,
/// would-establish, ATMs)`. Eq. 5.7.
pub(crate) fn sub_svo<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    if !RELATIVE_SUBJECT.matches(link.label()) {
        return None;
    }
    let object = link
        .right_word()
        .verb_group()
        .into_iter()
        .find_map(|verb| verb.right_by(OBJECT))?;
    Some(ConceptualRelation::SubSvo {
        subject: Concept::noun(link.left_word()),
        verb: Concept::verb(link.right_word()),
        object: Concept::noun(object),
    })
}

/// `football -Ss- ceased -TO- to -I- exist` gives `Svo(football, ceased,
/// exist)`. Eq. 6.3.
pub(crate) fn infinitive_svo<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    if !SUBJECT.matches_any(link.label()) {
        return None;
    }
    let object =
        link.right_word().right_by(TO)?.right_by(TO_INFINITIVE)?;
    Some(ConceptualRelation::Svo {
        subject: Concept::noun(link.left_word()),
        verb: Concept::verb(link.right_word()),
        object: Concept::verb(object),
    })
}

/// `he -Ss- began -Pg- singing` gives `Svo(he, began, singing)`. Eq. 6.4.
pub(crate) fn participle_svo<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    if !SUBJECT.matches_any(link.label()) {
        return None;
    }
    let object = link.right_word().right_by(PRESENT_PARTICIPLE)?;
    Some(ConceptualRelation::Svo {
        subject: Concept::noun(link.left_word()),
        verb: Concept::verb(link.right_word()),
        object: Concept::verb(object),
    })
}

/// `Hungary -YS- 's -D- business` gives `Possessive(Hungary, business)`.
/// Eq. 5.8.
pub(crate) fn possessive<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    if !POSSESSOR.matches_any(link.label()) {
        return None;
    }
    let possessed = link.right_word().right_by(DETERMINER)?;
    Some(ConceptualRelation::Possessive {
        possessor: Concept::noun(link.left_word()),
        possessed: Concept::noun(possessed),
    })
}

/// `invested -MVi- to -I- establish -O- ATMs` gives `Purpose(invested,
/// to-establish, ATMs)`. Eq. 6.1.
pub(crate) fn purpose<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    if !PURPOSE.matches(link.label()) {
        return None;
    }
    let verb = link.right_word().right_by(TO_INFINITIVE)?;
    let object = verb.right_by(OBJECT)?;
    Some(ConceptualRelation::Purpose {
        base_verb: Concept::verb(link.left_word()),
        purpose_verb: Concept::verb(verb),
        purpose_object: Concept::noun(object),
    })
}

/// `big -A- dog` gives `Attribute(dog, big)`. Eq. 6.5.
pub(crate) fn adjective<'l>(link: Link<'l>) -> Option<FoundRelation<'l>> {
    ADJECTIVE.matches(link.label()).then(|| {
        ConceptualRelation::Attribute {
            locus: Concept::noun(link.right_word()),
            attribute: Concept::single(link.left_word()),
        }
    })
}

/// `stock -AN- market` gives `Attribute(stock-market, stock)`. Eq. 6.6.
/// The locus holds the modifier too, because the noun group follows `AN`.
pub(crate) fn noun_modifier<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    NOUN_MODIFIER.matches(link.label()).then(|| {
        ConceptualRelation::Attribute {
            locus: Concept::noun(link.right_word()),
            attribute: Concept::single(link.left_word()),
        }
    })
}

/// `was -Osm- owner -Mf- of -Js- Fox` gives `ObjectOfObject(owner, Fox)`.
/// Eq. 6.9.
pub(crate) fn object_of_object<'l>(
    link: Link<'l>,
) -> Option<FoundRelation<'l>> {
    if !BE_OBJECT.matches_any(link.label()) {
        return None;
    }
    let object = link.right_word();
    let object_of_object = NOUN_PREPOSITION
        .iter()
        .find_map(|&pattern| object.right_by(pattern))?
        .right_by(PREPOSITION_OBJECT)?;
    Some(ConceptualRelation::ObjectOfObject {
        object: Concept::noun(object),
        object_of_object: Concept::noun(object_of_object),
    })
}

/// The subject of the verb group that `verb` belongs to.
fn subject_of(verb: Word<'_, English>) -> Option<Word<'_, English>> {
    verb.verb_group().into_iter().find_map(|word| {
        SUBJECT.iter().find_map(|&pattern| word.left_by(pattern))
    })
}
