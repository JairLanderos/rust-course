# Exercise 3.3: Battery monitor

Ferris the robot reports its battery level while it charges and works. The program in `src/main.rs` is almost right, but it doesn't compile, and even once it does, it doesn't report the right numbers yet.

Make it print exactly this report:

```text
Ferris: battery at 80%
Ferris: battery at 95%
Ferris: battery at 65%
Battery went from 65% to 0%
Ferris: battery at 0%
```

## Tasks

The comments in `src/main.rs` mark where each task is.

1. **Show the status without giving the name away.** `show_status` only needs to *read* the robot's name, but right now `main` loses it after the first call.
2. **Make charging count.** `charge` must change the battery level that `main` sees.
3. **Remember the old level.** `before` should hold the level from before the big drain. Make it work without changing what the line prints.
4. **Implement `drain`.** Take `amount` away from the level, but never go below 0.

## Rules

- Don't change the numbers or the order of the calls in `main`.
- Don't use `.clone()`: the point is to borrow.

## Check your work

From the workspace root:

```bash
cargo test -p s03-l03-references-borrowing
```

The exercise is done when the test passes. You can also run the program to see its report:

```bash
cargo run -p s03-l03-references-borrowing
```

## Hints

<details>
<summary>Hint for task 1</summary>

A function that only reads a value can take a shared reference: `&String` instead of `String`. The caller then passes `&robot`.

</details>

<details>
<summary>Hint for task 2</summary>

The compiler may suggest declaring the parameter `mut`. That makes the program compile, but look at the second line of the report: `charge` would only change its own copy of the number. To change `main`'s `battery`, `charge` needs a mutable reference to it.

</details>

<details>
<summary>Hint for task 3</summary>

`before` is a shared reference to `battery`, and it's still used after `drain` changes `battery`. A reader and a writer can't overlap. Do you really need a *reference* to the old level, or just the old number?

</details>

<details>
<summary>Hint for task 4</summary>

Use `*level` to read and change the value behind the reference, and an `if` to decide whether the result would go below 0.

</details>
