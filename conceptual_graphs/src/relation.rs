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
    /// The relation's name in the tuple notation, such as `Agent`.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Agent { .. } => "Agent",
            Self::Object { .. } => "Object",
            Self::Manner { .. } => "Manner",
            Self::Prepositional { .. } => "Prepositional",
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

/// Fuchs's tuple notation, such as `Agent(girl, eats)`.
impl<C: fmt::Display> fmt::Display for ConceptualRelation<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments = self
            .arguments()
            .iter()
            .map(|concept| concept.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}({arguments})", self.name())
    }
}

#[cfg(test)]
mod tests {
    use crate::ConceptualRelation;

    #[test]
    fn map_visits_concepts_in_field_order() {
        let mut visited = Vec::new();
        ConceptualRelation::Prepositional {
            head: 'a',
            preposition: 'b',
            object: 'c',
        }
        .map(|concept| visited.push(concept));

        assert_eq!(visited, ['a', 'b', 'c']);
    }
}
