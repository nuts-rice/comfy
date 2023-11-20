# v0.3.0

**This release contains breaking changes, most notably around the
`GameContext` and setup of the main game loop.** It's likely most games will
be affected, but the changes needed are very small and should help us a
lot in the long run.

If you run into any issues, please either create an issue on GitHub, or ping
@darth on Discord.

**Read the section about `GameContext` below for more information.** But
in practice you should be able to just figure out how to fix things from
the compile errors and finding a relevant example. The new game setup is
quite a bit simpler and more ergonomic.

[The full game loop
example](https://github.com/darthdeus/comfy/blob/master/comfy/examples/full_game_loop.rs)
contains a detailed description of how the main loop of Comfy is setup and
shows what the macros expand to. With `v0.3.0` there is no need for lifetimes
or `GameContext` and users are now much more easily able to define their own `main`
without macros.

## No more `GameContext` and lifetimes around `comfy_game!(...)`

One of the most controversial topics after Comfy's release was
`GameContext` vs `EngineContext`, and the the associated lifetimes with
`make_context` and overall the amount of boilerplate.

The initial stance was that `GameContext` is a good thing, as it allows more
flexibility and allows users to pass around a single struct. But things have
changed.

**In Comfy `v0.2.0` we moved a lot of things into globals, which made almost all
use cases around `EngineContext` obsolete and unnecessary.** As it is right now,
users shouldn't really need to pass around `EngineContext` anymore unless they
need something highly specific, such as disabling window resizing or changing
mouse behavior throught `winit`'s `Window` that can be accessed through
`c.renderer`. Everything else should be accessible through globals.

There are a few fields left on `EngineContext` that are mainly for internal use
and are mostly pending refactoring/deletion. We haven't odne this yet because
we're using some of it in our games, and doing it slowly allows us to keep our
games running against Comfy's `master` branch basically at all times. This
might seem like a selfish reason, but a big benefit users get from Comfy is
that we're using it to build games, and we're not locked to some older or
special fork version of the engine. In fact, with almost every change I make to
the engine I also update our games (most of the time just using `path` override
at all times) to ensure nothing serious broke, as the examples don't
necessarily cover everything.

TL;DR:

- Make sure to run your game with `--features dev` in development if you're using shaders
  to get hot reload errors displayed in game.
- `simple_game!(...)` should remain largely unaffected.
- Your `comfy_game!(...)` now works around a single type that acts both as a
  state object, as well as the game loop, and that can be passed around.
- `comfy_game!(...)` no longer requires `GameContext`
- `comfy_game!(...)` now accepts a single parameter, a type that implements a
  `GameLoop` trait. [The physics
  example](https://github.com/darthdeus/comfy/blob/master/comfy/examples/physics.rs)
  is a good starting point on how to implement this trait, but it should be
  mostly self-evident.
- `GameLoop` requires `new(c: &mut EngineState) -> Self` constructor, which
  acts as early game initialization.
- `GameLoop` implements a single `update(c: &mut EngineContext)` that ticks every frame.

Note that the `GameLoop` trait requires a `fn new(c: &mut EngineState) -> Self`
constructor, even though it is not theoretically needed if the user defines
their own `main` and doesn't use any macros. This is so that the
`comfy_game!(...)` macro wouldn't fail with cryptic error messages and users
can simply rely on implementing all functions of the `GameLoop` trait.

## User defined fragment shaders with uniforms

Starting from this version Comfy now supports user defined fragment shaders
with custom uniforms. Right now we only support `f32` uniforms, but this will
get expanded relatively soon.

The API is intentionally kept a bit low level and simple, as we already have
some use cases for Comfy via FFI. While this isn't a primary goal of the
engine, it is a use case that should very much remain supported, and as such
we'll try to make the lower level APIs FFI friendly. Higher level wrappers
(e.g. with RAII/Drop trait) will be added afterwards. Users should feel free to
create their own higher level wrappers around Comfy's low level primitives!

New shader related functions (see their individual docstrings & [fragment shader example](https://github.com/darthdeus/comfy/blob/master/comfy/examples/fragment-shader.rs) for more information):

- `create_shader`: Create a new shader from source code.
- `create_reloadable_shader`: Create a new hot reloadable shader from source code & path.
- `update_shader`: Update the source code of a shader. Intended for users who want to hot reload shaders manually.
- `use_shader`: Use a given shader for rendering from now on.
- `use_default_shader`: Switch back to the default shader.
- `set_uniform_f32`: Set a `f32` uniform value.

## Other changes

- Removed `(COMFY ENGINE)` from the title. This is now only shown in `--features dev`
  where `(Comfy Engine DEV BUILD)` is appended to the title. This can be useful for tiling
  window managers like i3 to automatically float windows with this title, e.g.
  `for_window [title=".*Comfy Engine DEV BUIL:D.*"] floating enable`.
- Notable upgrades: `wgpu 0.16.3 -> 0.17.1`, `egui 0.22.0 -> 0.23.0`. The
  `egui` upgrade is somewhat important, as `egui::plot` got moved into a
  separate `egui_plot` crate that Comfy now re-exports.
- Added `--feature git-version` that embeds the current git commit hash
  into the binary at build time. Note that this will make compilation fail
  if `cargo build` is run without there being any git history. See [the
  version
  example](https://github.com/darthdeus/comfy/blob/master/comfy/examples/version.rs)
  for details.
- Removed `--feature lua` and `mlua` integration. This was mainly a remnant of NANOVOID
  but was never implemented properly and missed a lot of bindings. If we do end up wanting
  to have official `mlua` bindings I'd rather that be done in a more principled approach
  where we make sure things are exported in a consistent way.

We're also introducing experimental render targets. This is a feature that
isn't yet complete, and there are some issues with it, but since merging it
doesn't really affect/break existing code it'll be included in v0.3 so that we
don't end up with long running feature branches for no reason. [There is an
example showcasing how this feature will
work](https://github.com/darthdeus/comfy/blob/render-targets/comfy/examples/render-target.rs),
but it's very likely we'll have breaking changes around this API, and would
like to discourage people from depending on this functionality in any way for
now. But do feel free to play around with it!

# v0.2.0

The main change in this release is that `EngineContext` is not necessary to
pass around anymore. This should simplify a lot of the confusion, as the #1
question about Comfy from _many_ people was about `GameContext` and
`EngineContext` and why are there two any why do we even need them.

Since Comfy already uses globals for many things, it makes sense to just
embrace this fully and move the remaining things to globals as well. Many
of the values provided in `EngineContext` were already available through
globals and just re-exported, so this shouldn't be a huge issue.

Comfy will still use `EngineContext` internally for some things, but this
won't be re-exported to the users as everything should be accessible by
other means.

List of removed things and where to find them now:

- `c.delta` -> `delta()`. This is likely going to be something that most users
  (including us) will re-export into their `GameContext/GameState` anyway.
- `c.world()` -> `world()`. ECS world already lived in a single instance, it's
  now moved into a single global.
- `c.commands()` -> `commands()`. Same as above.
- `c.cooldowns()` -> `cooldowns()`. This might be worth re-exporting into
  `GameContext` if accessed frequently, but in either way there's no extra
  overhead compared to before.
- `c.mouse_world` -> `mouse_world()`. This already existed before, and may
  also be worth re-exporting anyway.
- `c.egui` -> `egui()`. Note that before this was a field, now it's a
  global function. Though in this case `egui::Context` is already
  internally `Arc<Mutex<ContextImpl>>`, so this function is actually very
  cheap to call as it just returns a `&'static egui::Context` :)
- `c.egui_wants_mouse` -> `egui().wants_pointer_input()`
- `c.config` -> `game_config()` and `game_config_mut()`.
- `c.cached_loader.borrow_mut()` -> `cached_loader_mut()` (for `&` just
  omit `_mut`).
- similarly `c.changes.borrow_mut()` -> `changes()` and `c.notifications.borrow_mut()` -> `notifications()`.
  The last three were undocumented and are not really intended for public
  use yet, but documenting them here anyway.

NOTE: Comfy still includes many APIs which are not currently documented but are
still exposed. The current goal is to work through codebase and cleanup some
odd bits and document them at the same time. If you find something that is not
mentioned on the website or any examples, it's very likely to change in the
future.

As a secondary note, it should be noted that comfy is still _very early_ on in
its lifecycle. Comfy will do its best not to break existing games, but we may
need to iterate on some ideas, and some of them might be controversial, such
as the use of globals.

Comfy is not a project ran by RFCs, and while we do appreciate feedback, some
things have to be figured out by actually using the tool to build games,
running benchmarks, and making decisions based on real world usage.

In our usage of Comfy we've found many things that many in the Rust community
would consider "bad ideas" to be incredible boosts in ergonomics and
productivity. This is something that seems to happen more often than not, and
as such we're not really afraid to make changes like the above where a large
portion of the state is moved into globals. If you find the above change
unacceptable and what we had before "good", maybe take a look at the source
code and see how many globals we already had :)

That being said, the #1 priority of Comfy is and always will be making real
games. If any changes we make become problematic _in real world use cases_,
please do report these. If you think something is slow, please submit a
benchmark showing this. Comfy has enough examples using all of the systems, and
a builtin integration with Tracy, so it should be easy to extend. We do care
about reasonable games performing well on consumer hardware, but we do not care
about being the fastest at rendering 500k particles.

Our own games are not locked behind on an ancient version of Comfy, and we're
doing our best to stay up to date with the latest changes, to make sure things
are actually working smoothly.

## Bloom

Comfy `v0.1.0` had bloom turned on by default. This turned out to be quite
problematic on older integrated GPUs as some users reported, as the builtin
bloom does 20 blur passes :)

In `v0.2.0` bloom is now turned off by default. You can still enable it by
calling `game_config_mut().bloom_enabled = true;`. There's also a [new
example](https://github.com/darthdeus/comfy/blob/master/comfy/examples/bloom.rs)
that showcases bloom and how it can be configured.

## Chromatic aberration

Comfy `v0.1.0` also had chromatic aberration enabled by default, but
considering this isn't even a documented feature and the API for using it is
quite ugly we turned it off for now in `v0.2.0`. I don't think there's any
chance anyone actually used it, but if you did, it'll come back soon I promise.

Post processing is one of the things that should improve after `v0.2.0` is out,
and we'll be able to add more effects and make them easier to use.

## Minor changes:

- `GameConfig` is no longer `Copy`. This shouldn't really affect anyone in
  any way, as it was behind a `RefCell` anyway.

## Next up

The global namespace is currently polluted by a lot of things. The next
`v0.3.0` release will focus on cleaning this up and making some things more
directly accessible (e.g. some globals which are now currently not public).
