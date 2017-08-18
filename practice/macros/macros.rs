
// ===========================================================================
// macro_rules! defines a function much like fn vec
macro_rules! vec {
    /*
     * # Matching
     *
     * This is like a `match` expression arm but on Rust syntax trees. The
     * pattern on the left side of => is known as a `matcher`. These have
     * their own little grammer within the language (think C++ templates).
     *
     * The matcher `$x:expr` will match any Rust expression, binding to the
     * metavariable `$x`. The identifier `expr` is a 'fragment specifier'.
     *
     * `$(...),*` will match zero or more expressions, comma-separated.
     *
     * Aside from special matcher syntax, any Rust tokens that appear in a
     * matcher must match exactly.
     *
     * # Expansion
     *
     * The right-hand side of a macro rule is ordinary Rust syntax but we
     * can splice in bits of syntax captured by the matcher.
     */
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Aside from special matcher syntax
