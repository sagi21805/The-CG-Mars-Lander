use std::fmt;

use crate::ConceptId;

relation_enum! {
    /// A relation between concepts, from Fuchs's set (`theory.pdf`,
    /// Appendix B, §5.1 and chapter 6). A graph stores it with
    /// [`ConceptId`]s, and rules build it with the concepts themselves.
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
        /// Fuchs's `sub-SVO`, the same for a clause inside the sentence,
        /// as in "a business that would establish ATMs".
        SubSvo { subject: C, verb: C, object: C },
        /// Fuchs's `possessive`, as in "Hungary's business".
        Possessive { possessor: C, possessed: C },
        /// Fuchs's `purpose`, as in "invested to establish ATMs".
        Purpose { base_verb: C, purpose_verb: C, purpose_object: C },
        /// Sowa's ATTR, as in "a big dog".
        Attribute { locus: C, attribute: C },
        /// Fuchs's `OO`, the object of an object, as in "was the owner of
        /// Fox".
        ObjectOfObject { object: C, object_of_object: C },
    }
}

impl<C: Copy> ConceptualRelation<C> {
    /// The agent and the object that an `Svo` or a `SubSvo` folds into one
    /// tuple (`theory.pdf`, §4.4 step 6 and §5.1).
    ///
    /// `Svo(girl, eats, pie) -> [Agent(girl, eats), Object(eats, pie)]`
    pub(crate) fn agent_and_object(&self) -> Option<[Self; 2]> {
        let (Self::Svo {
            subject,
            verb,
            object,
        }
        | Self::SubSvo {
            subject,
            verb,
            object,
        }) = *self
        else {
            return None;
        };
        Some([
            Self::Agent {
                actor: subject,
                action: verb,
            },
            Self::Object {
                action: verb,
                object,
            },
        ])
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
