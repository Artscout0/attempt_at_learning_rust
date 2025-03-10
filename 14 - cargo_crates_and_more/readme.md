# Cargo, crates, and more details overall

## different profiles for dev and release

In the Cargo.toml file, you can specify a lot of things.
A lot of things are good by default, and for most projects you won't need to change them.

The kinda imortant things to remember are that:

- you can define different profiles, such as dev and release
- you can make them do things like compilation slightly differently.

Let's look at the following example:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

This will make it so that when you build for development, it'll build quickly, but it won't be perectly optimised.
When you instead build for release, it'll take longer to build, but it'll be as optimised as the compiler can optimise it (and remember, the compiler is almost always smarter than you)

This also happens to be the default behaviour.
If we change the 0 for a 1, it'll slightly increase compilation time, but slightly more optimise the build.

Full list of configuations available [here](https://doc.rust-lang.org/cargo/reference/profiles.html)

## crates.io

You can, without too many difficuties, publish a crate to crates.io. This will allow anyone to use the crates you made.

Before you publish, here are some good practices:

### good comments

This:

```rs
pub fn add_1(x: i32) -> i32 {
    x + 1
}
```

this is kinda unreadable.
Add one, sure! but add one to what? and add one of what? Pineaples?

But this:

```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

is readable. You know exactly what this does. Who else knows exactly what it does? Crates.io. They will make such documentative comments readable to the rustaceans.

You can also run

```sh
cargo doc --open
```

that will build a small html page that will display what will later be displayed on crates.io, and information about the crate.

Some commontly used sections are the examples, in the `# Examples` section, the panics, in the `# Panics` section, the errors in the `# Errors` section, and safety in the `# Safety` section (if the function is unsafe).

A useful thing about examples is that they will also be run as tests if you run `cargo test`.

Also, you can document from inside if you use `//!` instead of `///`.  This is useful to comment the crate itself and such.

### public api concerns

If you've used PHP with cargo, you've likely ended up with something like `use devname\modulename\class\some_other_stupid_thing`.

In rust, you can export a public function with `pub use`.

Basically, this will allow you to access specific types in a module by default. This will make it easier to access.

### metadata

You need to give a crate a name, cause two crates can't share a name, and they work first come first served basis.
You'll then need a license. If you don't want to use a premade license, you use license-file. You can then add a description, edition, and version.

This will end with something looking like

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```
