This respository contains a prototype for the new name resolution
algorithm I would like to propose (and eventually implement) for Rust.
The best way to get a feel for what it permits is to just
[read the tests](src/test.rs).

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
4. Globs fully supported, behavior is "what you expect" (well,
   what *I* expect, at least).
5. No guessing. Any possible ambiguities yield an error.

Some niggling details:

1. What IS a duplicate?
  - In the current model, `use foo::*; use foo:*;` yields an error, as
    does `use foo:f; use foo::f;`. It wouldn't be too hard, I think,
    to permit both of those, but it'd be very hard to permit `use
    foo::f;` and `use bar::f;` where `bar::f` is in fact a re-export
    of `f`.  In other words, we can compare for "the same path"
    shallowly, but not deeply. I opted not to compare for it at all.
1. Privacy
  - I don't model privacy yet but I would like to incorporate it.
  - It cannot be fully separated from name resolution.
  - General plan is to say that the EXPORTS from a module exclude
    private items except when exporting to submodules
  - Note that, in this model, `pub use` kind of conflates exporting
    and privacy. That is, one can imagine a `export` statement that
    permits you to re-export from a submodule to other submodules,
    without the result being `pub`. In general, the name `pub use`
    feels a bit awkward.

General overview of the algoritm:

- We are constructing a binding map `BINDINGS(m) = id -> BINDING` for
  each module `m`. The bindings map maps identifiers (`id`) to a
  `BINDING`, which is something like:
  
      BINDING = ITEM(id)      // An item declared in the map (in this prototype,
                                 either a module or struct)
              | PUBUSE(path)  // A re-exported path, also visible from outside module.
              | USE(path)     // An imported path, not visible from outside module.
              
  In reality, each binding is somewhat more complicated, because it
  contains intermediate state that is needed during name resolution to
  detect ambiguous bindings and manage the prioritization rules.  See
  the struct `Binding` in
  [src/nameresolution.rs](src/nameresolution.rs).
- We begin with a *SEED* phase that adds entries into `BINDINGS(m)` for
  each thing with an explicit name (that is, ignoring globs). This phase
  can yield an error if two declarations with the same name are detected.
- We proceed to a *SATURATE* phase that handles globs. The basic idea
  is to iterate over every glob like:
  
      mod m {
        ...
        use foo::bar::*;
        ...
      }
      
  For each such glob, we will try to resolve the base path `foo::bar`
  from within the module `m` using the current state of the `BINDINGS`
  table (in this case, the base path is absolute, but it could also be
  relative, e.g., `self::foo::bar`).  This resolution can yield one of
  three things:
  
  - A *hard error*, which basically means a cycle was encountered.
  - A *soft error*, which means that no name was encountered, possibly
    because the saturation phase is still incomplete.
  - *Success*, in which case we have a specific item.
  
  The distinction between *hard* and *soft* errors is that a hard
  error can never be recovered from. In contrast, as the saturation
  phase proceeds, more names will be added to `BINDINGS`, and hence
  soft errors may disappear over time. In the current prototype,
  we abort on hard error, but we could also report it and then remove
  the `use` statement from further consideration.
  
  If the result is success, then `foo::bar` should have mapped to a
  module (otherwise we have another *hard error*). We can then look at
  the current exports from this module and copy them into the current
  module. For each export `foo::bar::x`, we will consider adding an
  entry to `BINDINGS(m)` that looks like `x -> PUBUSE(foo::bar::x)`.
  I say *consider* because there are several cases in which we do
  not want to add the bindings:
  - If there is already a binding for `x`, and that binding originated
    from a non-glob, then the non-glob has priority. So we do nothing.
  - If there is already a binding for `x`, and that binding originated
    from the *same glob use*, then this addition is a no-op, and we can do
    nothing. This can occur because the SATURATION phase is an
    interative process.
  - If there is already a binding for `x`, and that binding
    originated from *another glob use*, then we report a *hard error*.
  - Finally, if there is *no binding* for `x`, then we just add the
    binding and proceed.

  Note that the *SATURATE* phase is *monotonic*:
  - That is, once a binding is added to `BINDINGS(m)`, that binding is
    never changed. If another competing binding of equal priority is
    encountered, we simply get an error. This is important because we
    will iteratively grow the bindings table, and hence if it were to
    change non-monotonically, the intermediate steps might be invalid.
- Once *SATURATE* is complete, we can resolve paths simply by walking
  over the links. At that point, if some identifier is not found in
  the binding table, we can simply report an error (whereas before
  that was a soft error that was ignored).
- One thing we may choose to do (which I do not do in the prototype)
  is to check that all `pub use` (and all `use`) statements can be
  successfully resolved once `SATURATE` is complete. This would detect
  cycles like `mod a { pub use b::T; } mod b { pub use a::T; }`.  In
  the prototype, such cycles are only detected when we actually
  attempt to resolve a path like `a::T`.

### ISSUES

- Relative precedence of macros vs explicit declarations
- Paths relative to types and type/value namespace -- resolve can't
  know which is which!
