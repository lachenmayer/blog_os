# `lachenmayer/blog_os`

Following along with the [Writing an OS in Rust](https://os.phil-opp.com/) course.

## Part 1: [Freestanding Rust binary](https://os.phil-opp.com/freestanding-rust-binary/)

I want to avoid going off the beaten path, so I don't get stuck down the line for some obscure reason.

In the very first command, `cargo new blog_os --bin --edition 2018`, I feel like I could get rid of the `--edition 2018` though... I feel like this is something I can always change afterwards, and I could maybe try out some new stuff if I leave it out. Let's see if it bites me in the ass.

OK, to get started, the first hello world build: `cargo build`, `./target/debug/blog_os`, works. `cargo run` also works.

Add `#![no_std]`, get the expected ```cannot find macro `println` in this scope``` error.

Once I remove the print, I already get off the beaten track:

```
error: `#[panic_handler]` function required, but not found

error: unwinding panics are not supported without std
```

This is already different to the errors mentioned in the post (which mentions `eh_personality`).

Changing the edition to 2018 doesn't seem to change anything, this is just based on the compiler version. Ah well, it might only be a difference in error message. It looks like there are some relatively helpful suggestions in the current error message.

Add the panic handler -- I get some error I don't understand from rust-analyzer, but let's see if we can get around this later. (I wonder if rust-analyzer doesn't understand that we've got `#![no_std]`?)

The error related to the missing panic handler is now gone though, so that's a success.

Add the required `panic = "abort"` lines to `Cargo.toml`. (It's so nice that all of these options are properly autocompleted instantly... Really feels like a dev environment that _works_!)

Again, the error message is different (```error: using `fn main` requires the standard library```), but it's clearly just a better version of the error message mentioned in the post (```error: requires `start` lang_item```).

Learn about `crt0`. Suggests a nice definition of a runtime: everything that runs before `main`.

Add `#![no_main]`. Again, super nice that all of these are properly auto-completed. I immediately get squiggles on `main`: ```function `main` is never used```, as it should be. Get rid of it.

Skip adding the `thumbv7em-none-eabihf` target, because we'll be defining our own in the next section.

Try `cargo rustc -- -C link-args="-e __start -static -nostartfiles"`, it runs fine -- ie. does nothing, as expected. (Or is it? `_start` is `loop {}`, should it not run forever?)

Ok nice, easy start so far.
