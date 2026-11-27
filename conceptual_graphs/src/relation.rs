use std::fmt;

use crate::ConceptId;

/// A relation between concepts, from Fuchs's set (`theory.pdf`,
/// Appendix B and §5.1). A graph stores it with [`ConceptId`]s, and
/// rules build it with the concepts themselves.
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
}

impl<C> ConceptualRelation<C> {
    /// The relation's name, such as `Agent`.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Agent { .. } => "Agent",
            Self::Object { .. } => "Object",
            Self::Manner { .. } => "Manner",
            Self::Prepositional { .. } => "Prepositional",
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
