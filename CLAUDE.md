# CLAUDE.md

## Purpose

The-CG-Mars-Lander is a research project. It aims to give AI models a **deterministic** way to read and query English text, as an alternative to purely statistical retrieval.

Pipeline:

1. **Parse** English text, both sources and queries, with a **link grammar** parser.
2. **Encode** each linkage automatically as a **conceptual graph** (CG).
3. **Store** the graphs in an **associative database**. The database is a hierarchy ordered by specificity: a general graph sits above the more specific graphs that contain it.
4. **Query** by parsing the question into a CG the same way, then locating it in the hierarchy. Matching is graph generalization/specialization, not embedding similarity.

The same input must always give the same output. Avoid anything non-deterministic in the core pipeline: no LLM calls, no sampling, no unordered-iteration dependence, no time- or locale-dependent behavior. Where a parser returns multiple linkages, ranking and tie-breaking must be explicit and stable.

## Repository state

Early stage. The `conceptual_graphs` crate holds the conceptual graph types. The rest are foundations:

| Path | What it is |
|---|---|
| `conceptual_graphs/` | The conceptual graph types (`ConceptualGraph`, `Concept`, `Context`, `ConceptualRelation`), built on link-grammar's `Sentence` and `Word`. |
| `link-grammar-rust-bindings/` | Git submodule (`sagi21805/link-grammar-rust-bindings`). Rust bindings: `link-grammar-sys` (raw FFI) and `link-grammar` (safe API). It has its own nested submodule, upstream C `link-grammar` 5.13.2. |
| `sources/link-grammar.pdf` | Sleator & Temperley, *Parsing English with a Link Grammar* (1991). The original link grammar paper. |
| `sources/conceptual-graphs.pdf` | Sowa, *Conceptual Graphs* (Handbook of Knowledge Representation, ch. 5). The CG notation and its logic (CGIF, Common Logic mapping). |
| `sources/theory.pdf` | Fuchs, *Practical Natural Language Processing Question Answering Using Graphs* (UCSC PhD thesis, 2004). The closest prior work to this project. It covers the associative database idea and the graph QA pipeline. |

Read the PDFs (`pdftotext` works) before designing the CG encoding or the database hierarchy. The design decisions should trace back to them.

## Language and tooling

- Rust, edition 2024, stable toolchain. Follow the `rust-style` skill for all Rust code.
- The root `Cargo.toml` is a workspace whose only member is `conceptual_graphs`, which depends on `link-grammar` by path (`link-grammar-rust-bindings/link-grammar`). The workspace excludes the submodule, which has its own workspace.

## Setup and commands

```sh
git submodule update --init --recursive   # required after a plain clone; there are nested submodules
```

Commands, run from `link-grammar-rust-bindings/`:

```sh
cargo build                  # see link-grammar/README.md for the `bundled` feature and dictionaries
cargo test
cargo xtask regen            # regenerate bindgen output and the flex lexer (needs libclang and flex)
cargo xtask package-check    # verify the crate builds from its published files
```

Do not run `cargo xtask regen` unless the upstream submodule tag moved.

## Design notes

These are the intended concepts. Update this section as decisions are made.

- **Link grammar output** is a set of labeled links between words (`S`, `O`, `D`, `MV`, ...) plus the connector subscripts. The mapping from link types to CG relations is the central design problem. It should be a documented, table-driven mapping rather than ad-hoc code, so it stays inspectable and deterministic.
- **Conceptual graphs** are bipartite: concept nodes (type, referent) and relation nodes. Use CGIF/Common Logic from Sowa as the reference for semantics. Keep an internal graph type independent from any serialization format.
- **Concept types** need a type hierarchy (a lattice) so generalization works. Decide the source: WordNet-style hypernyms, a fixed ontology, or derived from the corpus. This choice dominates retrieval quality.
- **Associative database**: graphs are indexed by the sub-graph/generalization partial order. A query is placed in the hierarchy and its specializations are retrieved. Preserve provenance (source document and sentence) for every stored graph, so answers are traceable.
- **Evaluation**: define a small gold set early (sentence, expected CG, question, expected answer) and run it as a regression test. Determinism makes exact-match testing possible, so use it.

## Conventions

- Keep the core library free of network and model dependencies.
- Prefer snapshot tests (sentence -> linkage -> CG -> serialized form) for the encoding. Any change to the link-to-relation mapping should show up as a reviewable diff.
- Cite the relevant paper section in a comment when an implementation choice follows one.
