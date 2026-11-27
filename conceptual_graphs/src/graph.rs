use std::fmt;
use std::ops::Index;

use link_grammar::{Linkage, Sentence};

use crate::link_rules::{
    FoundRelation, adverb_after_verb, adverb_before_verb, object,
    preposition, relative_subject, subject,
};
use crate::{Concept, ConceptualRelation};

/// One sentence as a conceptual graph, in Fuchs's tuple form. Each
/// relation refers to its concepts by [`ConceptId`], so a concept that
/// takes part in several relations exists once.
pub struct ConceptualGraph<'l> {
    pub sentence: &'l Sentence<'l>,
    pub concepts: Vec<Concept<'l>>,
    pub relations: Vec<ConceptualRelation>,
}

/// A concept's position in [`ConceptualGraph::concepts`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConceptId(pub u16);

impl<'l> ConceptualGraph<'l> {
    /// A graph of `sentence` with no concepts or relations.
    pub fn new(sentence: &'l Sentence<'l>) -> Self {
        Self {
            sentence,
            concepts: Vec::new(),
            relations: Vec::new(),
        }
    }

    /// The graph Fuchs's rules build from `linkage`, a linkage of
    /// `sentence`. Each rule tries every link. Relations come out grouped
    /// by rule, in the order of the calls below, and in link order within
    /// a rule.
    pub fn from_linkage(
        sentence: &'l Sentence<'l>,
        linkage: &'l Linkage<'l>,
    ) -> Self {
        let mut graph = Self::new(sentence);
        graph.add_relations(linkage.links().filter_map(subject));
        graph.add_relations(linkage.links().filter_map(relative_subject));
        graph.add_relations(linkage.links().filter_map(object));
        graph.add_relations(linkage.links().filter_map(adverb_after_verb));
        graph
            .add_relations(linkage.links().filter_map(adverb_before_verb));
        graph.add_relations(linkage.links().filter_map(preposition));
        graph
    }

    /// Adds `concept` and returns its id. A concept with the same words
    /// is added once, so the second call returns the first id and keeps
    /// the first concept's context.
    pub fn add_concept(&mut self, concept: Concept<'l>) -> ConceptId {
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
        found: impl Iterator<Item = FoundRelation<'l>>,
    ) {
        for found in found {
            let relation = found.map(|concept| self.add_concept(concept));
            self.relations.push(relation);
        }
    }
}

impl<'l> Index<ConceptId> for ConceptualGraph<'l> {
    type Output = Concept<'l>;

    fn index(&self, id: ConceptId) -> &Concept<'l> {
        &self.concepts[usize::from(id.0)]
    }
}

/// One relation per line in Fuchs's tuple notation, such as
/// `Agent(girl, eats)`.
impl fmt::Display for ConceptualGraph<'_> {
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
