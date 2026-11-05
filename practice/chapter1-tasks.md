# Chapter 1 practice — 10 tasks

Book: *The Impatient Programmer's Guide to Bevy and Rust*, chapter 1 (PDF pages 12–35).
Bevy **0.20**.

## How to use this file

- Do the tasks in order. Each one uses the ideas from the tasks before it.
- **Predict first, then run.** Write your guess down *before* you run the code.
  A wrong guess teaches you more than no guess.
- You write all the code. Bring me your answers and code, and I will review them.
- Tick the box when you finish a task: `[ ]` → `[x]`.
- If a doc link gives 404, use the docs.rs search box. Bevy sometimes moves
  types between modules.

### Doc links used everywhere

Bevy API search (type any name, e.g. `Timer`):
https://docs.rs/bevy/0.20/bevy/
The real source of truth for your version.

Bevy Quick Start:
https://bevy.org/learn/quick-start/introduction/
The official short intro to App, ECS, plugins and resources.

Unofficial Bevy Cheat Book:
https://bevy-cheatbook.github.io/
The best explanations of *concepts*. **Warning:** written for older Bevy, so some
code will not compile in 0.20. Read it for ideas, not code.

---

## Task 0 — Setup (not counted)

- [ ] Make a new project next to this one, e.g. `~/Projects/bevy-ch1-practice`.
- [ ] Add `bevy = "0.20"` to `Cargo.toml` (book, page 12).
- [ ] Copy the `src/assets` folder from `bevy-learning-clean` into the new project.
- [ ] `cargo run` once with an empty `main`. The first build is slow (Bevy is big). That is normal.

Read:

Cargo — creating a new package
https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html
What `cargo new` makes and why.

---

## Task 1 — The window and the camera

**Concept:** `App`, `DefaultPlugins`, the `Startup` schedule, `Camera2d`, `ClearColor`.
How does a window get on screen, and why is it empty?

**Do:**
1. Make an `App` with `DefaultPlugins` and one `Startup` system that spawns `Camera2d`.
2. Add `ClearColor` with any color you like (not white or black).
3. Change the window **title** and **size**. Hint: `DefaultPlugins` already holds a
   `WindowPlugin`. You change it the same way the book changes `AssetPlugin`
   (page 26). Look up `WindowPlugin`, `primary_window` and `Window`.
   You can chain `.set(...)` twice.

**Predict, then test:**
- a) Comment out `commands.spawn(Camera2d)`. What will the window show? Why?
- b) Spawn **two** `Camera2d`s. What happens? Read the terminal output too.
- c) Remove `DefaultPlugins` completely. Does a window open? What does `.run()` do then?

**Answer in your own words:**
- What is the job of the camera? What is the job of `ClearColor`?
- What does "Startup system" mean? How many times does it run?

Read:

Bevy Quick Start — Apps
https://bevy.org/learn/quick-start/getting-started/apps/
What `App::new()`, `add_plugins` and `run()` do.

Example: window settings (Bevy 0.20)
https://github.com/bevyengine/bevy/blob/v0.20.0/examples/window/window_settings.rs
Shows the `WindowPlugin { primary_window: ... }` pattern for step 3.

`ClearColor` in the API
https://docs.rs/bevy/0.20/bevy/?search=ClearColor
Shows that it is a *resource*: one global value for the whole app.

---

## Task 2 — Entities, components and coordinates

**Concept:** an entity is only an ID. Components are the data attached to it.
`commands.spawn((...))` takes a tuple of components. Bevy's 2D coordinate system.

**Do:**
1. Make a `Player` marker component (an empty struct with `#[derive(Component)]`).
2. Spawn the `"@"` text with `Player` at `Vec3::ZERO`, as in the book (page 20).
3. Spawn **three more** `"@"`s **without** `Player`, at `(200, 0)`, `(0, 200)`
   and `(-200, -100)`. Give each one a different `TextColor`.
4. `commands.spawn(...)` gives back something. Save it with `.id()` and print it with
   `info!("{:?}", id)`.

**Predict, then test:**
- a) Before you run it: draw a small sketch. Where will each `"@"` appear?
  Where is `(0, 0)` on the screen? Does `+y` go up or down?
- b) Make the window smaller. Do the `"@"`s move on the screen? Why?
- c) Two entities at the same position: which one is drawn on top?
  Then try `z = 1.0` on one of them.

**Answer in your own words:**
- Fill in this table for your world:

  | Entity ID | Components it carries |
  |---|---|
  | ... | ... |

- Why is an **empty** struct useful as a component?

Read:

Bevy Quick Start — ECS
https://bevy.org/learn/quick-start/getting-started/ecs/
The official explanation of Entity / Component / System.

Cheat Book — Coordinate system
https://bevy-cheatbook.github.io/fundamentals/coords.html
Where `(0,0)` is, which way `y` goes, and what `z` does in 2D.

Game Programming Patterns — Component
https://gameprogrammingpatterns.com/component.html
Why games split one object into many small parts. No Bevy here, only the idea.

---

## Task 3 — The game loop and time

**Concept:** after `Startup`, Bevy runs the `Update` schedule **every frame**,
forever. `Res<Time>` tells you how long the last frame took.

**Do:**
1. Write a system `print_time(time: Res<Time>)` that logs `time.delta_secs()`
   and `time.elapsed_secs()`.
2. Register it in `Update`. Run it for a few seconds and look at the numbers.
3. Now register **the same system** in `Startup` too.

**Predict, then test:**
- a) What number do you expect for `delta_secs()`? Work out your FPS
  (frames per second) from it.
- b) Move or resize the window while it runs. Does `delta_secs()` change?
- c) How many lines does the `Startup` copy print? What is its `delta_secs()` value?

**Answer in your own words:**
- What is a "frame"? What happens in one frame (see the book diagram, page 13)?
- Why is `Time` a **resource** and not a component?

Read:

Game Programming Patterns — Game Loop
https://gameprogrammingpatterns.com/game-loop.html
**The most important link in this file.** Read the "Fixed update, variable rendering"
part up to "Variable time step". This is why `delta` exists.

Bevy Quick Start — Resources
https://bevy.org/learn/quick-start/getting-started/resources/
Resources versus components.

`Time` in the API
https://docs.rs/bevy/0.20/bevy/time/struct.Time.html
Find the difference between `delta()` and `delta_secs()`. Look at the return types.

---

## Task 4 — Keyboard input and movement

**Concept:** `Res<ButtonInput<KeyCode>>`, `Single<&mut Transform, With<Player>>`,
`Vec2`, `normalize()`, speed × delta.

**Do:**
1. Write `move_player` from the book (page 22) **from memory**. Look at the book only when you are stuck.
2. Add **WASD** keys as well as the arrow keys.
3. Add a key that moves the player back to `(0, 0)`. Use `just_pressed`, not `pressed`.

**Predict, then test:**
- a) Remove `.normalize()`. Is diagonal movement faster? How much faster?
  (Hint: what is the length of the vector `(1, 1)`?)
- b) Put `.normalize()` back. Remove `* time.delta_secs()`. Then set the speed to `5.0`.
  Does the speed now depend on your FPS? Why is that bad for players
  with slow or fast computers?
- c) Press Left and Right at the same time. What happens? Why? (This is the
  "why not `else if`" question from your comments.)
- d) What is the difference between `pressed` and `just_pressed`? Use both
  and check.

**Answer in your own words:**
- In `Single<&mut Transform, With<Player>>`, what does each part mean?
  What does `With<Player>` *filter out*?

Read:

`ButtonInput` in the API
https://docs.rs/bevy/0.20/bevy/input/struct.ButtonInput.html
Compare `pressed`, `just_pressed` and `just_released`.

Example: keyboard input (Bevy 0.20)
https://github.com/bevyengine/bevy/blob/v0.20.0/examples/input/keyboard_input.rs
A small official example to compare with your system.

Math is Fun — Unit vector
https://www.mathsisfun.com/algebra/vector-unit.html
What "normalize" means, with pictures. Short.

---

## Task 5 — Queries: Single vs Query, and what With does

**Concept:** a query asks the world: "give me all entities that have these components".
Filters (`With<T>`) make the list smaller.

**Do:**
1. Make a copy of your `move_player` that uses `Query<&mut Transform, With<Player>>`
   and `single_mut()` with `let ... else` (like the book, page 30).
2. Spawn a **second** entity that also has `Player`.

**Predict, then test:**
- a) With 2 players, what does the `Single` version do? What does the
  `single_mut()` version do? Is it the same? Read the terminal output.
- b) Change the query to loop over **all** players with `for mut t in &mut query { ... }`.
  Do both players move now?
- c) Remove `With<Player>` from the loop version, so it is `Query<&mut Transform>`.
  Which entities move now? **Look closely at the screen. Why does it look
  like nothing moves?** (Think about what else has a `Transform`.)

**Answer in your own words:**
- When should you use `Single`, and when `Query` + a `for` loop?
- What does `let Ok(x) = ... else { return; };` do? Why is it there?

Read:

Cheat Book — Queries
https://bevy-cheatbook.github.io/programming/queries.html
Query, filters (`With` / `Without`), and single versus many.

`Single` in the API
https://docs.rs/bevy/0.20/bevy/ecs/system/struct.Single.html
Read what happens to the system when the entity is missing or there are two.

Rust by Example — let-else
https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html
The Rust syntax used in every system in chapter 1.

---

## Task 6 — Timers, without sprites yet

**Concept:** `Timer`, `TimerMode::Repeating` vs `Once`, `tick()`, `just_finished()`.
A newtype component (`struct BlinkTimer(Timer)`) with `Deref, DerefMut`.

This is the heart of animation. We learn it on simple text first.

**Do:**
1. Make a component `BlinkTimer(Timer)` with `#[derive(Component, Deref, DerefMut)]`.
2. Give the player `"@"` a `BlinkTimer` set to 0.5 seconds, repeating.
3. Write a system that **switches the player's `TextColor`** between two colors
   every time the timer finishes.

**Predict, then test:**
- a) Remove the `tick(...)` line. What happens? Why?
- b) Change `Repeating` to `Once`. What happens?
- c) Pass `time.delta() * 2` to `tick` instead of `time.delta()`. What happens?
- d) Remove `Deref, DerefMut` from the derive. Which line fails to compile?
  How do you fix it *without* putting them back? (Hint: `.0`)

**Answer in your own words:**
- Why does each entity need its **own** timer component, and not one global timer?
- Why does `tick` need `time.delta()`? What would go wrong with a frame counter
  ("change the color every 30 frames")?

Read:

`Timer` in the API
https://docs.rs/bevy/0.20/bevy/time/struct.Timer.html
Read `tick`, `just_finished`, `finished` and `reset`.

Example: timers (Bevy 0.20)
https://github.com/bevyengine/bevy/blob/v0.20.0/examples/time/timers.rs
Timers as components *and* as a resource. Compare both.

Rust by Example — New Type idiom
https://doc.rust-lang.org/rust-by-example/generics/new_types.html
Why wrap `Timer` inside your own struct.

Rust Book — the Deref trait
https://doc.rust-lang.org/book/ch15-02-deref.html
What `Deref` really does. Read the start, up to "Implicit Deref Coercions".

---

## Task 7 — The sprite sheet, by hand

**Concept:** a sprite sheet is one image with a grid of frames.
`TextureAtlasLayout::from_grid` describes the grid. `TextureAtlas.index` picks one cell.
`AssetServer::load` returns a `Handle`, not the image itself.

**Part A — on paper, no code:**
1. Open `character-spritesheet.png` in an image viewer. Find its size in pixels.
   How many 64×64 columns and rows does the image **really** have?
2. The book uses `from_grid(64, 9 columns, 12 rows)`. Which part of the image does
   that include? Which part does it leave out?
3. Work out these by hand: the index of row 10, column 0. The row and column of
   index `95`. The index of the last frame of the "Right" row.

**Part B — code:**
1. Change the `"@"` to a sprite with an atlas (book, pages 28–29). Show index 0.
2. Make the **number keys 1–4** choose the row (Up / Left / Down / Right), and
   **Space** move one column forward. This is a *manual* flipbook. You are
   the timer.

**Predict, then test:**
- a) Set `index` to a number larger than `9 * 12 - 1`. What happens?
- b) Use `from_grid` with **13** columns, but keep your old index math with
   `WALK_FRAMES = 9`. Which frames do you get now? Why?
- c) Load a file name that does not exist. Does the game crash? What do you see?

**Answer in your own words:**
- What is the difference between the **texture** (`Handle<Image>`), the
  **layout** (`Handle<TextureAtlasLayout>`) and the **index**?
- Why does `load()` give a handle and not the image right away?

Read:

What is a sprite sheet? (CodeAndWeb)
https://www.codeandweb.com/what-is-a-sprite-sheet
A short visual explanation of the whole idea. Watch the animation on the page.

`TextureAtlasLayout::from_grid` in the API
https://docs.rs/bevy/0.20/bevy/?search=TextureAtlasLayout
Read the parameters: `padding` and `offset` are the two `None`s in the book.

Example: sprite sheet (Bevy 0.20)
https://github.com/bevyengine/bevy/blob/v0.20.0/examples/2d/sprite_sheet.rs
The official version of this task. Compare it with yours *after* you finish.

LPC sprite sheet generator
https://liberatedpixelcup.github.io/Universal-LPC-Spritesheet-Character-Generator/
The tool from the book (page 25). Look at how the rows are ordered.

---

## Task 8 — Facing: enum, match and the row

**Concept:** the `Facing` enum, `match`, derived `PartialEq` / `Copy`,
choosing the row from the direction of movement.

**Do:**
1. Add `Facing` and `AnimationState { facing, moving, was_moving }` (book, pages 27–28).
2. In `move_player`, set `facing` from the direction. Use the "dominant axis"
   rule from the book (page 31).
3. Write `row_zero_based`, `row_start_index` and `atlas_index_for`.
4. For now, **no timer**: when the facing changes, only jump to column 0 of the new row.
   The player should *turn* but not *walk*.

**Predict, then test:**
- a) Hold Up + Right together. Which way does the player face? Why? Change
  `>` to `>=` in the dominant-axis check. What changes?
- b) Remove `Copy, Clone` from the `Facing` derive. Which lines fail to compile? Read
  the error message. What does it say about "move"?
- c) Remove `PartialEq` from the derive. What fails?

**Answer in your own words:**
- Why is `Facing` an **enum** and not four `bool` fields? (book, page 27)
- `atlas_index_for` uses `.min(WALK_FRAMES - 1)`. Give an input where this
  protects you.

Read:

Rust Book — Enums and match
https://doc.rust-lang.org/book/ch06-02-match.html
`match` and why it must cover every case.

Rust Book — Derivable traits
https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
What `Debug`, `Clone`, `Copy`, `PartialEq` and `Eq` each give you.

Rust Book — What is ownership (the "move" part)
https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
The reason why `Copy` matters in experiment b.

---

## Task 9 — The walking animation

**Concept:** put it all together: timer + atlas index + modulo + edge detection
(`just_started` / `just_stopped`).

**Do:**
1. Add `AnimationTimer` and write `animate_player` (book, page 32).
   Try it from your own understanding first. Use the book to check.
2. Make it work in all 4 directions.
3. **Design change:** in the book, when you stop, the player freezes on the last
   frame. Change it so the player shows the **standing** frame (column 0) when stopped.

**Predict, then test:**
- a) Set `ANIM_DT` to `0.02`, then to `0.5`. Describe what you see. Which value
  looks right with `MOVE_SPEED = 140`? Why must these two values "match"?
- b) Delete the whole `just_started` branch (keep only the timer branch).
  Tap an arrow key **very quickly**. What does the player look like?
- c) The book has `timer.reset()` inside `just_started`. Your old project does **not**
  (`player.rs` line 189–190). Is it really needed? **Prove it**: think through the
  timer value frame by frame, then test. (I am not telling you the answer.)
- d) Line `anim.was_moving = anim.moving;` — move it to the **top** of the
  function. What breaks? Why?

**Answer in your own words:**
- Explain `(current_col + 1) % WALK_FRAMES` to someone who has never seen `%`.
- Why do we need `was_moving` at all? Could `animate_player` find out
  "just started" without it?

Read:

Game Programming Patterns — Update Method
https://gameprogrammingpatterns.com/update-method.html
"Each object updates a little each frame". This is what `animate_player` does.

Example: sprite animation (Bevy 0.20)
https://github.com/bevyengine/bevy/blob/v0.20.0/examples/2d/sprite_animation.rs
The official version also starts and stops on key press. Compare its design with the book's.

---

## Task 10 — Modules and plugins (final task)

**Concept:** `mod`, `pub`, `use crate::...`, the `Plugin` trait and `build()`,
and systems that run together in one tuple `(move_player, animate_player)`.

**Do:**
1. Move all player code into `player.rs` with a `PlayerPlugin` (book, pages 33–34).
   `main.rs` should only have the app, the camera and `.add_plugins(PlayerPlugin)`.
2. Make a **second** plugin by yourself, `CameraPlugin` in `camera.rs`, that owns
   the camera and the `ClearColor`.
3. **Final challenge — no book allowed:** add **sprint**. Holding **Shift** makes the
   player move 2× faster **and** makes the animation play 2× faster.
   Decide yourself: where does this logic go? Which component or constant changes?

**Predict, then test:**
- a) Remove `pub` from `struct PlayerPlugin`. What is the error, and in which file?
- b) Add `PlayerPlugin` **twice** in `main`. What happens?
- c) Register `move_player` and `animate_player` as two separate
  `.add_systems(Update, ...)` calls instead of one tuple. Does anything change?
  Does Bevy promise which one runs first? (Look up `.chain()`.)

**Answer in your own words:**
- What is a trait? What does `impl Plugin for PlayerPlugin` promise to Bevy?
- Draw the whole chapter 1 program as boxes and arrows:
  App → plugins → schedules → systems → components/resources.
  If you can draw this, you understand chapter 1.

Read:

Bevy Quick Start — Plugins
https://bevy.org/learn/quick-start/getting-started/plugins/
The official explanation of `Plugin` and `build`.

Rust Book — Defining modules
https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
`mod`, `pub` and privacy. This is the answer to experiment a.

Rust Book — Traits
https://doc.rust-lang.org/book/ch10-02-traits.html
What "implement a trait" means. Read up to "Default Implementations".

Cheat Book — System order
https://bevy-cheatbook.github.io/programming/system-order.html
Why Bevy runs systems in parallel, and how `.chain()` / `.before()` set the order.
