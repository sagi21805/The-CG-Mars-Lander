use conceptual_graphs::ConceptualGraph;
use link_grammar::{Dictionary, Language, Sentence};

/// The relations of the graph built from the first linkage of `text`, one
/// per item, in tuple notation.
fn tuples(text: &str) -> Vec<String> {
    let dictionary = Dictionary::new(Language::English)
        .expect("the bundled English dictionary loads");
    let mut sentence =
        Sentence::new(&dictionary, text).expect("the text has no NUL");
    sentence
        .parse()
        .expect("the sentence is short enough to parse");
    let linkage = sentence.linkage(0).expect("the sentence parses");
    ConceptualGraph::from_linkage(&sentence, &linkage)
        .to_string()
        .lines()
        .map(String::from)
        .collect()
}

#[test]
fn subject_and_object_become_agent_and_object() {
    assert_eq!(
        tuples("The girl eats pie."),
        ["Agent(girl, eats)", "Object(eats, pie)"],
    );
}

#[test]
fn number_becomes_the_quantity_of_its_noun() {
    assert_eq!(
        tuples("The girl eats five pies."),
        ["Agent(girl, eats)", "Object(eats, pies|five)"],
    );
}

#[test]
fn large_number_becomes_the_quantity_of_its_noun() {
    assert_eq!(
        tuples("He paid 5 million dollars."),
        ["Agent(he, paid)", "Object(paid, dollars|5-million)"],
    );
}

#[test]
fn auxiliary_joins_the_verb_concept() {
    assert_eq!(
        tuples("The girl doesn't eat pie."),
        ["Agent(girl, doesn't-eat)", "Object(doesn't-eat, pie)"],
    );
}

#[test]
fn perfect_tense_joins_the_verb_concept() {
    assert_eq!(
        tuples("The company has invested one million dollars in Hungary."),
        [
            "Agent(company, has-invested)",
            "Object(has-invested, dollars|one-million)",
            "Prepositional(has-invested, in, Hungary)",
        ],
    );
}

#[test]
fn adverb_after_verb_is_manner() {
    assert_eq!(
        tuples("A girl eats a pie fast."),
        [
            "Agent(girl, eats)",
            "Object(eats, pie)",
            "Manner(eats, fast)",
        ],
    );
}

#[test]
fn adverb_before_verb_is_manner() {
    assert_eq!(
        tuples("The privately managed fund invested money."),
        [
            "Agent(fund, invested)",
            "Object(invested, money)",
            "Manner(managed, privately)",
        ],
    );
}

#[test]
fn unmarked_adverb_is_not_manner() {
    assert_eq!(
        tuples("The girl never eats pie."),
        ["Agent(girl, eats)", "Object(eats, pie)"],
    );
}

#[test]
fn relative_clause_subject_is_agent() {
    assert_eq!(
        tuples(
            "The fund invested in a business that would establish ATMs \
             nationwide."
        ),
        [
            "Agent(fund, invested)",
            "Agent(business, would-establish)",
            "Object(would-establish, ATMs)",
            "Prepositional(invested, in, business)",
        ],
    );
}

#[test]
fn proper_noun_chain_is_one_concept() {
    assert_eq!(
        tuples("Rupert Murdoch owned Fox."),
        ["Agent(Rupert-Murdoch, owned)", "Object(owned, Fox)"],
    );
}

#[test]
fn rules_share_one_verb_concept() {
    assert_eq!(
        tuples("The girl didn't eat five pies in the kitchen."),
        [
            "Agent(girl, didn't-eat)",
            "Object(didn't-eat, pies|five)",
            "Prepositional(didn't-eat, in, kitchen)",
        ],
    );
}
