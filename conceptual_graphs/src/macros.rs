//! `macro_rules!` that other modules use, so `lib.rs` declares this module
//! first.

/// The relation enum as written, plus the methods that handle every
/// variant the same way, so a new relation needs only its variant.
///
/// - `name`, the variant's name, such as `Agent`.
/// - `field_names`, the names of its fields, in field order.
/// - `arguments`, the concepts it joins, in field order.
/// - `map`, the same relation with a function applied to each concept.
macro_rules! relation_enum {
    (
        $(#[$meta:meta])*
        pub enum $relation:ident<C = $concept:ty> {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident { $($field:ident: C),+ $(,)? }
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        pub enum $relation<C = $concept> {
            $(
                $(#[$variant_meta])*
                $variant { $($field: C),+ },
            )+
        }

        impl<C> $relation<C> {
            /// The relation's name, such as `Agent`.
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant { .. } => stringify!($variant),)+
                }
            }

            /// The names of the relation's fields, in field order.
            fn field_names(&self) -> &'static [&'static str] {
                match self {
                    $(
                        Self::$variant { .. } => {
                            &[$(stringify!($field)),+]
                        }
                    )+
                }
            }

            /// The concepts the relation joins, in field order.
            pub fn arguments(&self) -> Vec<&C> {
                match self {
                    $(
                        Self::$variant { $($field),+ } => {
                            vec![$($field),+]
                        }
                    )+
                }
            }

            /// The same relation with `f` applied to each concept, in
            /// field order.
            ///
            /// `Agent { actor: girl, action: eats } -> Agent { actor: C0,
            /// action: C1 }`
            pub fn map<D>(self, mut f: impl FnMut(C) -> D) -> $relation<D> {
                match self {
                    $(
                        Self::$variant { $($field),+ } => $relation::$variant {
                            $($field: f($field)),+
                        },
                    )+
                }
            }
        }
    };
}
