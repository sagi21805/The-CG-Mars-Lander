use std::fmt;
use std::ops::Index;

use link_grammar::{Linkage, Sentence};

use crate::link_rules::{FoundRelation, LINK_RULES};
use crate::{Concept, ConceptualRelation};

/// One sentence as a conceptual graph, in Fuchs's tuple form. Each
/// relation refers to its concepts by [`ConceptId`], so a concept that
/// takes part in several relations exists once.
///
/// ```
/// use conceptual_graphs::{
///     Concept, ConceptualGraph, ConceptualRelation, Context,
/// };
/// use link_grammar::{Dictionary, Language, Sentence};
///
/// let dictionary = Dictionary::new(Language::English)?;
/// let mut sentence =
///     Sentence::new(&dictionary, "The girl eats five pies.")?;
/// sentence.parse()?;
/// let linkage = sentence.linkage(0).expect("a parse");
/// let words: Vec<_> = linkage.words().collect();
/// // words[0] is LEFT-WALL, so `girl` is words[2].
///
/// let mut graph = ConceptualGraph::new(&sentence);
/// let girl = graph.add_concept(Concept {
///     words: vec![words[2]],
///     context: Context::default(),
/// });
/// let eats = graph.add_concept(Concept {
///     words: vec![words[3]],
///     context: Context::default(),
/// });
/// let pies = graph.add_concept(Concept {
///     words: vec![words[5]],
///     context: Context { quantity: vec![words[4]], ..Context::default() },
/// });
/// graph.relations.push(ConceptualRelation::Agent {
///     actor: girl,
///     action: eats,
/// });
/// graph.relations.push(ConceptualRelation::Object {
///     action: eats,
///     object: pies,
/// });
///
/// assert_eq!(
///     graph.to_string(),
///     "Agent(girl, eats)\nObject(eats, pies|five)",
/// );
/// # Ok::<(), link_grammar::Error>(())
/// ```
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
    /// by rule, in the rules' fixed order, and in link order within a
    /// rule.
    ///
    /// ```
    /// use conceptual_graphs::ConceptualGraph;
    /// use link_grammar::{Dictionary, Language, Sentence};
    ///
    /// let dictionary = Dictionary::new(Language::English)?;
    /// let mut sentence =
    ///     Sentence::new(&dictionary, "The girl doesn't eat pie.")?;
    /// sentence.parse()?;
    /// let linkage = sentence.linkage(0).expect("a parse");
    /// let graph = ConceptualGraph::from_linkage(&sentence, &linkage);
    ///
    /// assert_eq!(
    ///     graph.to_string(),
    ///     "Agent(girl, doesn't-eat)\nObject(doesn't-eat, pie)",
    /// );
    /// // `Agent` and `Object` share the verb concept.
    /// assert_eq!(graph.concepts.len(), 3);
    /// # Ok::<(), link_grammar::Error>(())
    /// ```
    pub fn from_linkage(
        sentence: &'l Sentence<'l>,
        linkage: &'l Linkage<'l>,
    ) -> Self {
        let mut graph = Self::new(sentence);
        let found = LINK_RULES
            .into_iter()
            .flat_map(|rule| linkage.links().filter_map(rule));
        for relation in found {
            graph.add_relation(relation);
        }
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

    /// Adds the concepts of `found`, merging any the graph already has,
    /// and the relation between them.
    fn add_relation(&mut self, found: FoundRelation<'l>) {
        let relation = found.map(|concept| self.add_concept(concept));
        self.relations.push(relation);
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
            let arguments = relation
                .arguments()
                .into_iter()
                .map(|&id| self[id].to_string())
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "{}({arguments})", relation.name())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use link_grammar::{Dictionary, Language, Sentence};

    use crate::{Concept, ConceptualGraph, ConceptualRelation, Context};

    /// "The girl eats pie.", where words[2] is `girl` and words[3] is
    /// `eats`.
    const SENTENCE: &str = "The girl eats pie.";

    #[test]
    fn adding_the_same_words_twice_returns_the_first_id() {
        let dictionary = Dictionary::new(Language::English)
            .expect("the bundled English dictionary loads");
        let mut sentence = Sentence::new(&dictionary, SENTENCE)
            .expect("the text has no NUL");
        sentence.parse().expect("the sentence parses");
        let linkage = sentence.linkage(0).expect("a parse");
        let words: Vec<_> = linkage.words().collect();

        let mut graph = ConceptualGraph::new(&sentence);
        let eats = Concept {
            words: vec![words[3]],
            context: Context::default(),
        };
        let first = graph.add_concept(eats.clone());
        let second = graph.add_concept(eats);

        assert_eq!(first, second);
        assert_eq!(graph.concepts.len(), 1);
    }

    #[test]
    fn negated_concept_displays_with_a_bang() {
        let dictionary = Dictionary::new(Language::English)
            .expect("the bundled English dictionary loads");
        let mut sentence = Sentence::new(&dictionary, SENTENCE)
            .expect("the text has no NUL");
        sentence.parse().expect("the sentence parses");
        let linkage = sentence.linkage(0).expect("a parse");
        let words: Vec<_> = linkage.words().collect();

        let mut graph = ConceptualGraph::new(&sentence);
        let girl = graph.add_concept(Concept {
            words: vec![words[2]],
            context: Context::default(),
        });
        let eats = graph.add_concept(Concept {
            words: vec![words[3]],
            context: Context {
                negated: true,
                ..Context::default()
            },
        });
        graph.relations.push(ConceptualRelation::Agent {
            actor: girl,
            action: eats,
        });

        assert_eq!(graph.to_string(), "Agent(girl, !eats)");
    }
}
