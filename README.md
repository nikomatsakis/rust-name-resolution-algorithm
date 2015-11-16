This repository contains a prototype for the new name resolution
algorithm I would like to propose (and eventually implement) for Rust.
It is a very flexible algorithm: globs are fully supported; macros can
be used and impored like anything else; references to macros do not
have to come "after" the definition of the macro in any sense; macros
can define items (even other macro definitions) that can then be
imported and used (even via globs).

Here are some of the interesting characteristics:

1. Explicit imports and structs take precedence over globs.
    - So if I have `use foo::*;` and `use bar::X;`, then the name `X`
      always resolves to `bar::X`, even if `foo` contains a member
      named `X`.
    - **However,** macro names can never conflict, even with a macro
      imported by a glob. So in the previous example, if `X` should be
      a macro, an error results.
2. It is always legal to import the same item through multiple paths
   (e.g., `use foo::X` and `use bar::X` are legal, so long as they
   both ultimately refer to the same `X`).
3. It is legal for two globs to conflict so long as the conflicting
   item is not referenced (e.g., a module can have both `use foo::*`
   and `use bar::*` even if both `foo` and `bar` define a struct `X`,
   so long as the module never refers to `X`).
   
The core algorithm is (I think) backwards compatible with Rust today
(modulo bugs in today's code that accept things they should not). But
there are a few places that the algorithm as implemented here diverge
from Rust which are probably not strictly backwards compatible (and
may not be desirable, depending on who you ask):

1. `use` statements define links that can be referenced from other
   modules. In other words, if a parent module has `use foo::Bar`,
   then it is legal for one of its children to do `use super::Bar`.
   This makes `use` and `pub use` statements completely analogous,
   except for the privacy of the resulting item.
2. Globs from an ancestor module import both public and private items.
   In other words, `use super::*` will bring in private items defined
   in your parent as well as public items. Note that this combines
   with point 1 to mean that `use super::*` effectively recreates your
   parent's naming environment completely.
   
### Caveats with the current state   
   
There are also some caveats (as of the time of this writing, anyway,
this summary may get out of date):

1. I don't model multiple namespaces yet. I'll get to that.
2. Privacy is not yet implemented. I may get to that.
3. Hygiene and macro arguments in general are not modeled. I probably
   won't get to that.

I don't foresee any horrible problems with any of these
things. However, multiple namespaces have surprised me in the past in
terms of their complexity, so I do intend to actually implement
THOSE. The others seem simple enough, I may do it if the whim strikes
me.

### How the algorithm works at a high level

The basic idea is that a fixed point expansion followed by a
verification step. We basically progressively process the code,
resolving macros, use statements, and globs until we reach a stable
state. We then check that the final result was self-consistent and
free of duplicate names and so forth.

#### Output of the algorithm

The output of the algorithm is a either an error or, upon success, a
new AST tree (actually, the implementation mutates the existing one in
place) and a set of bindings `{Name -> ItemId}` for each module.  Here
`ItemId` is an unambiguous identifier for everything in the AST. So if
I have a module like `mod foo { struct Bar; struct Baz; }`, then the
bindings for this module would be something like `{Bar -> 22, Baz ->
23}`, assuming that the ids assigned to the two structs `Bar` and
`Baz` were 22 and 23 respectively. Note that there is nothing which
says that this set of bindings must be unambigious. For example, we
might have two bindings for the same name, like `{Bar -> 44, Bar ->
66}`.

*Side note:* The actual implementation gives identifies a bit more
structure; they carry tags identifying the sort of item they refer to,
so the id of `Bar` would be something like `ItemId::Struct(22)`. This
is defined in [`ast.rs`][ast], which is also a kind of experimental
playground for other ways to struct rustc's own internal AST.

#### Intermediate state

The computation itself is a fixed-point computation that incrementally
builds up these binding sets. During this process, the binding sets
include tuples with slightly more information: `(Name, Precedence,
ItemId)`, where `Precedence` is either high (explicit declarations) or
low (glob). It also carries along a set called "fully expanded". This
is just a set of modules whose contents are fully expanded, meaning
that they contain no macro references. It is important to know this
because it affects when we can start processing globs (read on).

#### Algorithm pseudo-code

Here is the algorithm in pseudo-code. Note that this definition maps
quite cleanly to the function `resolve_and_expand` in the
[actual implementation][the code].

```
loop {
    expand macros where possible;
    seed names for all modules;
    glob names for all modules;
} until fixed point is reached;
verify all paths;
```

To see how the algorithm works, let's consider this example:

```rust
mod foo {
    use bar::*;
    use std::collections::HashMap;
    
    some_macro! { ... };
    
    struct MyStruct { }
}
mod bar {
    macro_rules! some_macro {
        ...
    }
}
```

Now let's go over each of the steps above. However, I'm going to go
over them in a slightly 

**Expand macros where possible.** The first we do is to try and expand
macros. For every macro reference, e.g. `some_macro! { .. }` in our
example above, we try to resolve the path (here, `some_macro`,
resolved relative to the current module). This may or may not
succeed. If it fails, it could be for a variety of reasons: there
might be no binding for `some_macro` (yet); there might be *multiple*
bindings for `b` (which indicates an error); or, the path might not
lead to a macro. Whatever the cause, if path resolution fails, we
don't report any errors just now, we just ignore it.

If however we *can* resolve the path, and it leads to a macro, then we
can do macro expansion here. However, and this is important, when we
expand the macro, we don't just replace the macro reference
completely. Instead, we leave behind a "husk" that contains the path
of the expanded macro. You can think of this as being similar to the
"expansion trace" that rustc tracks: for each bit of code that
resulted from macro expansion, we also remember the name of the macro
that was used to create it. We'll need this path in a later
step. (Note that in an actual implementation, you might choose to just
remember a set of paths for expanded macros, rather than leaving a
marker in the AST itself.)

This path also updates the `fully_expanded` set.

In [the code], this is the function `expand_macros`.

**Seed names for all modules.** The seed step walk creates bindings
for everything except globs. It works by traversing all the modules.
For declarations, it will add a simple binding from the declared name
to the declaration.  So, in our example above, if the `struct
MyStruct` declaration has id `X`, then we would add `(MyStruct, High,
StructId(X))` to the binding set for the module `foo`. Similarly, in
the module `bar`, we would also add `(some_macro, High, ModuleId(Y))`,
where `Y` is the id of the macro rules declaration for `some_macro`.

Explicit import statements deserve special mention. If we see a
non-glob `use`, such as `std::collections::HashMap`, then we first try
to resolve the path. If that succeeds, it will yield an item id `Z`,
and we can add an entry like `(HashMap, High, Z)`. However, if path
resolution *fails* for whatever reason, then we still add a
binding. But because we don't know the target id, this is a special
"placeholder" binding `(HashMap, High, 0)`, where `0` is a special
placeholder id. This placeholder binding doesn't count as a normal
binding and will be ignored for path resolution: it's purpose is to
ensure that we don't add glob bindings for `HashMap` in the next step.

In [the code], this is the function `seed_names`.

**Process globs.** Finally, we process globs. In the example, we see
the module `foo` that contains a glob `use bar::*`. To process this
glob, we would first try to resolve the path `bar` to a single,
unambigious item (ignoring errors, as usual). Presuming this succeeds,
and resolves to a module with id `M`, we then go over the bindings `{N
-> ID}` found within this module `M` as follows:

- *Macro definitions.* If `ID` refers to a macro definition, then we
  will add it to the containing module (e.g., `foo`) as a
  high-priority binding.
- *Non-macros.* For all other kinds of bindings, we add the binding
  to the importing module as a low-priority binding, but only if the
  following conditions are met:
  - The macro `M` that we are importing from has no unexpanded macros.
  - There is no high-priority binding for `N` within the importing module.
  
In [the code], this is the function `glob_names`.

**Verify all paths.** Ok, once we've fully completed expanding macros
and propagating names, we proceed to the last step, which is verifying
paths. This is the place where we actually report some
errors. Basically in this step we iterate down all the items and check
that all the paths we find can be resolved correctly. We also check
that definitions are unambigious. For example, continuing with our
example function above:

- For the `use a::b::c::*` import, we would verify that `a::b::c` resolves
  to a module.
- For `use std::collections::HashMap`, we would verify that this
  resolves to a single item (when multiple namespaces are supported,
  this would have to resolve to a single item in at least one namespace,
  and possibly zero items in the other).
- For `struct MyStruct`, we would verify that `self::MyStruct` leads
  to this struct definition. This ensures that there is no other
  `struct MyStruct` (for example).
- The macro reference `some_macro!` would presumably have been
  expanded in a prior step into some code. If it has not, that would
  be because the path `some_macro` cannot be resolved, and we would
  report an error. If it *HAS* been expanded, however, there will
  still be a "macro husk" lying around, as described in the previous
  step, containing the path `some_macro`. So we would resolve that
  husk and check that the path `some_macro` leads unambiguously to a
  single macro definition. (If this is the case, it must be the same
  definition that we used to expand.)

In [the code], this is the function `verify_paths`.

[the code]: src/resolve.rs
[ast]: src/ast.rs

### FAQ

#### Is this algorithm correct?

Well, correct is naturally a bit hard to define. I believe the
algorithm meets the expectations of Rust users and does logical
things. But I think we can also say something a bit stronger: the
result of the algorithm is *coherent*. Let me try to define what I
mean by that (I intend to make this more formal, but for now it's
rather hand-wavy). Let's consider an expanded form of the AST in which
all links are made explicit:

- Every path has a link describing the item it refers to
- Globs are expanded to include a list of explicit items that they
  bring into scope
- All macros are expanded, and code derived from expanding a macro
  includes some means to determine the macro it was expanded from (the
  "husk" in the algorithm above)

Note that this expanded form roughly corresponds to what the "semantic
analysis" step of a compiler normally does (or "resolve", as rustc
calls it). In this expanded, explicit form, we can define a fairly
simple notion of correctness. Basically, every path should in fact
lead to the destination that is specified (without encountering
ambigiuities along the way) and so forth. (Here I am assuming as
"obviously correct" a kind of naive path resolution algorithm that
walks to each module, inspects all the bindings that are within, and
selects the only matching one.)

I think of the algorithm as analogous to type inference: it infers all
this extra information that is elided from the original AST and
creates an explicit form. I believe that, if this inference succeeds,
then the final resulting program is guaranteed to be "resolve
correctly" (in the sense I described in the previous paragraph). You
can see that the "verify" step comes close to checking most of the
correctness requirements (and reporting an error if any of them fail).
For example, it checks that every path resolves to *something*
unambiguously. But it doesn't check the *full* set of properties,
because it doesn't check that the result of this resolution is in fact
the item we expected. This is because I believe that it is not
necessary to check what the path resolves to: if it resolves to just
one thing, it MUST be the thing we want. But it'd be nice to prove
that (or at least argue it in a somewhat formal fashion).

It would also be nice to make an argument that the "inference" step
preserves the user's intent. For example, we'd like to show that if a
solution is found, there is no other possible solution. Ideally, we'd
also show that if an error results, then there is no possible solution
to be found that respects the criteria.

#### Why do explicit items shadow globs?

Because it's better for forwards compatibility. It makes it less
likely that adding a public item will accidentally break a downstream
client. In other words, if someone does this:

```rust
use crate_x::*;
use crate_y::Foo;
struct Bar(Foo);
```

And in the meantime `crate_x` adds a new entry `Foo`, the downstream
create does not break. Still, it's not a guarantee. For example,
someone might do:

```rust
use crate_x::*;
use crate_y::*;
struct Bar(Foo);
```

and, in that case, they will get an error if `crate_x` later adds a
`Foo` binding. (Note though that if they were not referencing `Foo`,
there would be no problem.)

#### Why don't explicit macros take precedence over globs?

This is to avoid time-travel-like paradoxes that can otherwise occur
when expanding macros. The problem case is when we have a macro found
with a path like `a::b!`. Suppose that the containing module has a
`use foo::*;` which was the origin of the module `a`. But then the
macro *generates* a module `a` as well. Now we are in a quandry: to
load the macro in the first place, we had to use the `a` from the
glob, but that `a` was shadowed by code that the macro generated. This
is bad because that shadowed item should have *taken precedence* over
the `a` we got from the glob, but we can't go back and *undo* the
macro expansion we already did (and anyway, if we did so, we wouldn't
have the new definition for `a` anymore)! So we call it an error.

Now, there are other ways to resolve this. One would be to take a
"mutable view" on the set of bindings. That is, when a module expands,
we only consider the names that were in scope *at the time of
expansion*, which might later change. I wanted to avoid this because
it introduces *ordering* between macro expansions -- that is, suppose
I have to two macro expansions in a row, like `foo!(); bar!();`. Now
if I expand `foo` first, it may generate names that would conflict
with expanding `bar` -- but it would have been fine if I had just
expanded `bar` first! I want to preserve the Rust-y notion that the
order of declaration for items doesn't matter.

Another way of looking at is in terms of the expanded form I described
in the question about correctness: a mutable view means that the
"correctness" property of this expanded form would be a lot more
complex, because we would have to reason not about all the bindings we
see, but the bindings that were visible at the time of expansion.

#### Can we just remove macros from globs instead?

It has been proposed that we could keep the full inference rules if we
just removed macros from globs. But I don't think that works. Consider this
example:

```rust
mod a {
    use b::c::n
    n! { }
}
mod b {
    use c::*;
    macro_rules! m {
        () => {
            mod d {
                macro_rules! n { ... }
            }
        }
    }
    m! { }
}
mod c {
    mod d {
        macro_rules! n {
            ...
        }
    }
}
```

The problem here has to do with the macro reference `n!` invoked by
the module `a`. It is imported with the path `b::d::n`. We can resolve
this via the glob import from `c` but, once we expand the macro `m` in
module `b`, we see that in fact this module generates a
higher-precedence module that should have taken precedence over the
glob. So we are stuck in that we cannot freely expand macros in any
order. (Note: this is the test case
`banning_macro_globs_is_not_enough`.)

#### This seems too good to be true. What's the catch?

Not sure yet. I haven't really found any scenarios that fail, but I also
haven't tried the algorithm at scale.

