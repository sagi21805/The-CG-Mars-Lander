//! Fuchs's rules for English (`theory.pdf`, §4.4, §5.1 and §6.3).

mod concept;
mod link_rules;
mod word_walk;

use link_grammar::Linkage;
use link_grammar::english::English;

use self::link_rules::{
    adverb_after_verb, adverb_before_verb, object, preposition,
    relative_subject, subject,
};
use crate::{ConceptualGraph, LanguageRules};

impl LanguageRules for English {
    fn add_linkage<'l>(
        graph: &mut ConceptualGraph<'l, Self>,
        linkage: &'l Linkage<'l, Self>,
    ) {
        graph.add_relations(linkage.links().filter_map(subject));
        graph.add_relations(linkage.links().filter_map(relative_subject));
        graph.add_relations(linkage.links().filter_map(object));
        graph.add_relations(linkage.links().filter_map(adverb_after_verb));
        graph
            .add_relations(linkage.links().filter_map(adverb_before_verb));
        graph.add_relations(linkage.links().filter_map(preposition));
    }
}
