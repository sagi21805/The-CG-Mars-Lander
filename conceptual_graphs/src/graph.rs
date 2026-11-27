use std::fmt;
use std::ops::Index;

use link_grammar::english::English;
use link_grammar::{Language, Linkage, Sentence};

use crate::language_rules::FoundRelation;
use crate::{Concept, ConceptualRelation, LanguageRules};

/// One sentence as a conceptual graph, in Fuchs's tuple form. Each
/// relation refers to its concepts by [`ConceptId`], so a concept that
/// takes part in several relations exists once.
pub struct ConceptualGraph<'l, L: Language = English> {
    pub sentence: &'l Sentence<'l, L>,
    pub concepts: Vec<Concept<'l, L>>,
    pub relations: Vec<ConceptualRelation>,
}

/// A concept's position in [`ConceptualGraph::concepts`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConceptId(pub u16);

impl<'l, L: Language> ConceptualGraph<'l, L> {
    /// A graph of `sentence` with no concepts or relations.
    pub fn new(sentence: &'l Sentence<'l, L>) -> Self {
        Self {
            sentence,
            concepts: Vec::new(),
            relations: Vec::new(),
        }
    }

    /// Adds `concept` and returns its id. A concept with the same words
    /// is added once, so the second call returns the first id and keeps
    /// the first concept's context.
    pub fn add_concept(&mut self, concept: Concept<'l, L>) -> ConceptId {
        let index = self
            .concepts
            .iter()
            .position(|known| known.words == concept.words)
            .unwrap_or_else(|| {
                self.concepts.push(concept);
                self.concepts.len() - 1
            });
        ConceptId(
            u16::try_from(index)
                .expect("a sentence has fewer than 65536 concepts"),
        )
    }

    /// Adds each relation in `found` and its concepts, merging any
    /// concept the graph already has.
    pub(crate) fn add_relations(
        &mut self,
        found: impl Iterator<Item = FoundRelation<'l, L>>,
    ) {
        for found in found {
            let relation = found.map(|concept| self.add_concept(concept));
            self.relations.push(relation);
        }
    }
}

/// Works for any language with rules.
impl<'l, L: LanguageRules> ConceptualGraph<'l, L> {
    /// The graph the rules of `L` build from `linkage`, a linkage of
    /// `sentence`. Each rule tries every link. Relations come out grouped
    /// by rule, in the order the language calls its rules, and in link
    /// order within a rule.
    pub fn from_linkage(
        sentence: &'l Sentence<'l, L>,
        linkage: &'l Linkage<'l, L>,
    ) -> Self {
        let mut graph = Self::new(sentence);
        L::add_linkage(&mut graph, linkage);
        graph
    }
}

impl<'l, L: Language> Index<ConceptId> for ConceptualGraph<'l, L> {
    type Output = Concept<'l, L>;

    fn index(&self, id: ConceptId) -> &Concept<'l, L> {
        &self.concepts[usize::from(id.0)]
    }
}

/// One relation per line in Fuchs's tuple notation, such as
/// `Agent(girl, eats)`.
impl<L: Language> fmt::Display for ConceptualGraph<'_, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line, relation) in self.relations.iter().enumerate() {
            if line > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{}", relation.map(|id| &self[id]))?;
        }
        Ok(())
    }
}
