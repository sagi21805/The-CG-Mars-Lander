use conceptual_graphs::ConceptualGraph;
use link_grammar::{Dictionary, Language, Sentence};

fn main() {
    let text = "Today I went to the mall and bought iceream";
    let dictionary = Dictionary::new(Language::English)
        .expect("the bundled English dictionary loads");
    let mut sentence =
        Sentence::new(&dictionary, text).expect("the text has no NUL");
    sentence
        .parse()
        .expect("the sentence is short enough to parse");
    let linkage = sentence.linkage(0).expect("the sentence parses");
    let g = ConceptualGraph::from_linkage(&sentence, &linkage);

    println!("Graph: {}", g);
}
