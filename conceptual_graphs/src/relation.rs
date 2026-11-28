use std::fmt;

use crate::ConceptId;

/// A relation between concepts, from Fuchs's set (`theory.pdf`,
/// Appendix B, §5.1 and chapter 6). A graph stores it with [`ConceptId`]s,
/// and rules build it with the concepts themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConceptualRelation<C = ConceptId> {
    /// Sowa's AGNT, as in "the girl eats".
    Agent { actor: C, action: C },
    /// Sowa's OBJ, as in "eats pie".
    Object { action: C, object: C },
    /// Sowa's MANR, as in "eats fast".
    Manner { action: C, manner: C },
    /// Fuchs's `prepositional`, as in "invested in Hungary".
    Prepositional { head: C, preposition: C, object: C },
    /// Fuchs's `SVO`, an agent and an object in one tuple, as in "the
    /// girl eats pie".
    Svo { subject: C, verb: C, object: C },
    /// Fuchs's `sub-SVO`, the same for a clause inside the sentence, as
    /// in "a business that would establish ATMs".
    SubSvo { subject: C, verb: C, object: C },
    /// Fuchs's `possessive`, as in "Hungary's business".
    Possessive { possessor: C, possessed: C },
    /// Fuchs's `purpose`, as in "invested to establish ATMs".
    Purpose {
        base_verb: C,
        purpose_verb: C,
        purpose_object: C,
    },
    /// Sowa's ATTR, as in "a big dog".
    Attribute { locus: C, attribute: C },
    /// Fuchs's `OO`, the object of an object, as in "was the owner of
    /// Fox".
    ObjectOfObject { object: C, object_of_object: C },
}

impl<C> ConceptualRelation<C> {
    /// The relation's name, such as `Agent`.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Agent { .. } => "Agent",
            Self::Object { .. } => "Object",
            Self::Manner { .. } => "Manner",
            Self::Prepositional { .. } => "Prepositional",
            Self::Svo { .. } => "Svo",
            Self::SubSvo { .. } => "SubSvo",
            Self::Possessive { .. } => "Possessive",
            Self::Purpose { .. } => "Purpose",
            Self::Attribute { .. } => "Attribute",
            Self::ObjectOfObject { .. } => "ObjectOfObject",
        }
    }

    /// The names of the relation's fields, in field order.
    fn field_names(&self) -> &'static [&'static str] {
        match self {
            Self::Agent { .. } => &["actor", "action"],
            Self::Object { .. } => &["action", "object"],
            Self::Manner { .. } => &["action", "manner"],
            Self::Prepositional { .. } => {
                &["head", "preposition", "object"]
            }
            Self::Svo { .. } | Self::SubSvo { .. } => {
                &["subject", "verb", "object"]
            }
            Self::Possessive { .. } => &["possessor", "possessed"],
            Self::Purpose { .. } => {
                &["base_verb", "purpose_verb", "purpose_object"]
            }
            Self::Attribute { .. } => &["locus", "attribute"],
            Self::ObjectOfObject { .. } => &["object", "object_of_object"],
        }
    }

    /// The concepts the relation joins, in field order.
    pub fn arguments(&self) -> Vec<&C> {
        match self {
            Self::Agent { actor, action } => vec![actor, action],
            Self::Object { action, object } => vec![action, object],
            Self::Manner { action, manner } => vec![action, manner],
            Self::Prepositional {
                head,
                preposition,
                object,
            } => vec![head, preposition, object],
            Self::Svo {
                subject,
                verb,
                object,
            }
            | Self::SubSvo {
                subject,
                verb,
                object,
            } => vec![subject, verb, object],
            Self::Possessive {
                possessor,
                possessed,
            } => vec![possessor, possessed],
            Self::Purpose {
                base_verb,
                purpose_verb,
                purpose_object,
            } => vec![base_verb, purpose_verb, purpose_object],
            Self::Attribute { locus, attribute } => {
                vec![locus, attribute]
            }
            Self::ObjectOfObject {
                object,
                object_of_object,
            } => vec![object, object_of_object],
        }
    }

    /// The same relation with `f` applied to each concept, in field
    /// order.
    ///
    /// `Agent { actor: girl, action: eats } -> Agent { actor: C0, action:
    /// C1 }`
    pub fn map<D>(
        self,
        mut f: impl FnMut(C) -> D,
    ) -> ConceptualRelation<D> {
        match self {
            Self::Agent { actor, action } => ConceptualRelation::Agent {
                actor: f(actor),
                action: f(action),
            },
            Self::Object { action, object } => {
                ConceptualRelation::Object {
                    action: f(action),
                    object: f(object),
                }
            }
            Self::Manner { action, manner } => {
                ConceptualRelation::Manner {
                    action: f(action),
                    manner: f(manner),
                }
            }
            Self::Prepositional {
                head,
                preposition,
                object,
            } => ConceptualRelation::Prepositional {
                head: f(head),
                preposition: f(preposition),
                object: f(object),
            },
            Self::Svo {
                subject,
                verb,
                object,
            } => ConceptualRelation::Svo {
                subject: f(subject),
                verb: f(verb),
                object: f(object),
            },
            Self::SubSvo {
                subject,
                verb,
                object,
            } => ConceptualRelation::SubSvo {
                subject: f(subject),
                verb: f(verb),
                object: f(object),
            },
            Self::Possessive {
                possessor,
                possessed,
            } => ConceptualRelation::Possessive {
                possessor: f(possessor),
                possessed: f(possessed),
            },
            Self::Purpose {
                base_verb,
                purpose_verb,
                purpose_object,
            } => ConceptualRelation::Purpose {
                base_verb: f(base_verb),
                purpose_verb: f(purpose_verb),
                purpose_object: f(purpose_object),
            },
            Self::Attribute { locus, attribute } => {
                ConceptualRelation::Attribute {
                    locus: f(locus),
                    attribute: f(attribute),
                }
            }
            Self::ObjectOfObject {
                object,
                object_of_object,
            } => ConceptualRelation::ObjectOfObject {
                object: f(object),
                object_of_object: f(object_of_object),
            },
        }
    }
}

/// A struct, such as `Agent { actor: girl, action: eats }`. `{:#}` puts
/// each field on its own line.
impl<C: fmt::Display> fmt::Display for ConceptualRelation<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed = f.debug_struct(self.name());
        for (field, concept) in
            self.field_names().iter().zip(self.arguments())
        {
            printed.field(field, &format_args!("{concept}"));
        }
        printed.finish()
    }
}
