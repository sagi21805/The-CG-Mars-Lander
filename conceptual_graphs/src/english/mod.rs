//! Fuchs's rules for English (`theory.pdf`, §4.4, §5.1 and §6.2 to §6.4).

mod concept;
mod link_rules;
mod word_walk;

use link_grammar::Linkage;
use link_grammar::english::English;

use self::link_rules::{
    adjective, adverb_after_verb, adverb_before_verb, infinitive_svo,
    noun_modifier, object, object_of_object, participle_svo, possessive,
    preposition, purpose, relative_subject, sub_svo, subject, svo,
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
        graph.add_relations(linkage.links().filter_map(svo));
        graph.add_relations(linkage.links().filter_map(infinitive_svo));
        graph.add_relations(linkage.links().filter_map(participle_svo));
        graph.add_relations(linkage.links().filter_map(sub_svo));
        graph.add_relations(linkage.links().filter_map(possessive));
        graph.add_relations(linkage.links().filter_map(purpose));
        graph.add_relations(linkage.links().filter_map(adjective));
        graph.add_relations(linkage.links().filter_map(noun_modifier));
        graph.add_relations(linkage.links().filter_map(object_of_object));
    }
}
