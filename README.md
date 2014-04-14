This respository contains a prototype for the new name resolution
algorithm I would like to propose (and eventually implement) for Rust.

The overview of the algorithm is roughly that:

1. Explicit declarations, imports (`use`), or re-exports (`pub use`)
   always override glob imports and re-exports.
2. Other than the above rule, ambiguity is generally forbidden:
   - Cannot declare two explicit items with the same name
   - Cannot declare an item with the same name as an explicit import/re-export
   - Cannot have two glob imports that would import the same name,
     or a glob import and a glob-re-export
3. If resolving a path ever leads to a cycle, an error is reported.

General goals:

1. *Adding* new public items to a library:
   - can never change the results of downstream name resolution except
     by causing an error
   - an error is only possible through glob imports from multiple
     libraries, in which case ambiguities may arise.
2. Adding new private items to a library can NEVER cause an error for
   users (see note about privacy below).
3. Ordering of use statements with respect to declarations or to one
   another doesn't matter.
4. Globs fully supported, behavior is "what you expect".
5. No guessing. Any possible ambiguities yield an error.

Some niggling details:

1. Privacy
   - I don't model privacy yet but I would like to incorporate it.
   - It cannot be fully separated from name resolution.
   - General plan is to say that the EXPORTS from a module exclude
     private items except when exporting to submodules

Note that, in this model, `pub use` kind of conflates exporting and
privacy. That is, one can imagine a `export` statement that permits
you to re-export from a submodule to other submodules, without the
result being `pub`. In general, the name `pub use` feels a bit
awkward.


