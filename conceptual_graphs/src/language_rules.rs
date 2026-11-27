use link_grammar::english::English;
use link_grammar::{Language, Linkage};

use crate::{Concept, ConceptualGraph, ConceptualRelation};

/// The rules that turn one language's linkages into a graph.
pub trait LanguageRules: Language {
    /// Adds the relations the language's rules find in `linkage`.
    fn add_linkage<'l>(
        graph: &mut ConceptualGraph<'l, Self>,
        linkage: &'l Linkage<'l, Self>,
    );
}

/// A relation a rule found, holding its concepts instead of their ids.
pub(crate) type FoundRelation<'l, L = English> =
    ConceptualRelation<Concept<'l, L>>;
