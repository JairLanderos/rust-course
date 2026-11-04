# Rust Course Workspace

This is your workspace for the Rust course. Everything you build in the course lives here, in one Cargo workspace.

## Layout

| Path | What it is |
|---|---|
| `sections/` | Your packages, one per lesson: the ones you create by following a lesson, and the exercises you start. Every directory at `sections/*/*` is a workspace member. |
| `exercises/` | Exercise templates, one per lesson that has an exercise. Don't edit them: copy one into `sections/` to start it. |
| `solutions/` | Reference solutions, mirroring `exercises/`. Try the exercise first! |
| `capstone/` | Reserved for the capstone project. Not a workspace member. |
| `tools/` | Scripts used to maintain the course. |

## Conventions

- **Package names** are `sNN-lMM-<slug>`: section, lesson, and a short name. For example, Lesson 3.3's package is `s03-l03-references-borrowing`. The directory name and the package name are always the same.
- **Packages live in their section's directory**, for example `sections/s03-ownership/s03-l03-references-borrowing/`.
- **Everything compiles.** When you finish a lesson (and its exercise), `cargo build --workspace` and `cargo test --workspace` succeed.

## Starting an exercise

From this directory, copy the template into `sections/`, then read its `README.md`. For example, for Lesson 3.3:

```bash
mkdir -p sections/s03-ownership
cp -r exercises/s03-ownership/s03-l03-references-borrowing sections/s03-ownership/
```

The exercise is done when its tests pass:

```bash
cargo test -p s03-l03-references-borrowing
```

## Getting updates

New and corrected exercises arrive when you pull from the course repository:

```bash
git pull course main
```

Your work in `sections/` is never touched by an update.
