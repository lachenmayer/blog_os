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

## Part 2: [A Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/)

Learn about [POST](https://en.wikipedia.org/wiki/Power-on_self-test), BIOS v UEFI.

I find it funny that we still boot from 16-bit "real mode", to 32-bit "protected mode", and then 64-bit "long mode", for backwards compability reasons with the 8086, which came out over 40 years ago.

Also lol, there is an "[unreal mode](https://en.wikipedia.org/wiki/Unreal_mode)". It seems like it's basically a 32-bit mode without any of the memory protection/paging/virtual memory of the protected mode? Interesting. Does this still work with modern processors?

Luckily, we don't have to write our own bootloader. This is where we get into freaky-deaky hardware land, where everything is magic, undocumented and/or hardware-specific.

Read about GRUB -- this brings back horrible flashbacks. In my experience, if you ever need to mess around in GRUB when using Linux, you are usually in deep shit, and you can expect to spend the next couple of hours trying to unfuck your computer instead of doing what you wanted to do. But hey, we're writing a new kernel, this one is _never_ going to be broken. Luckily we're going to swerve GRUB here. From the drawbacks listed here, it does seem like GRUB sucks a lot. Are there serious alternatives? (Perhaps _written in Rust_?)

The post mentions something about nightly Rust, I hope we can also swerve that, 6 years after the fact.

Add `x86_64-blog_os.json`. Copy and paste here, instead of typing it out as I usually do, because I (a) cba to actually understand what's going on here, and (b) the `data_layout` field looks particularly wild ([docs](https://llvm.org/docs/LangRef.html#data-layout) -- nothing magic, but do not want to have to understand this in detail or god forbid have to debug this). Everything else is fairly self-explanatory.

Learn about [LLD](https://lld.llvm.org/). This is something I've seen floating around (again usually in compiler errors that indicate that you're more deeply screwed than you ever wanted), but never actually knew what it was. Turns out it's a linker shipped with LLVM.

Learn about the ["red zone"](https://os.phil-opp.com/red-zone/). Neat optimization for functions which don't call any other functions.

Very interesting rationale for _not_ enabling SIMD in kernels (`-mmx,-sse`): when enabled, the kernel would need to save SIMD state to main memory on every system call / hardware interrupt. SIMD state can be very large (512-1600 bytes), so this would be really slow. Interesting, had never thought about that.

Interestingly, we then can't use hardware floats, because they use the MMX registers. `+soft-float` emulates these in software. I assume this isn't going to be much of an issue in practise, I don't really know where we'd need floats in a kernel.

Ok, when I run `cargo build --target x86_64-blog_os.json`, I get the ```error[E0463]: can't find crate for `core` ``` error as expected.

I also get a note about ```the `x86_64-blog_os` target may not be installed```, which is a bit more worrying, let's see if that's expected.

Ok, so we do need nightly because of `build-std`. Fine. I had previously installed Rust via Homebrew, uninstall that and install Rustup instead.

First time I try the build it gives me a helpful error telling me to add `rust-src`, which is exactly what I need.

Next try, the build works. Nice!

Interesting that `core` depends on 14 crates. Pretty cool.

Ok, getting to the first interesting part, writing to screen using VGA. Baby's first `unsafe` as well.

Tiny code change, I define `let offset = i as isize * 2` instead of repeating it. Can't see why this would break.

## Running our Kernel

Install `bootloader = "0.9"`. Was tempted to install the latest version, but the post explicitly warns against it. Thanks! (Would have been a classic off-the-rail moment...)

Nice that they built the `bootimage` tool too, this seems like it would have been a major pain in the ass to do manually.

Install QEMU using Homebrew (`brew install qemu`), let's see if this works.

`qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin` does open up a window, that's already a success!

It's blank though, hmm... Let's see, what did I mess up in the VGA code?

```rust
let vga_buffer = 0xb8000 as *mut u8;
```

I used `0x8000` instead of `0xb8000`, duh...

Ok, I now get lovely teal text, nice!

Also add the `cargo run` shortcut for the next chapters, very handy.

## Part 3: [VGA Text Mode](https://os.phil-opp.com/vga-text-mode/)

Learn about [code page 437](https://en.wikipedia.org/wiki/Code_page_437), the encoding used in VGA text mode. This has all those box drawing glyphs, as well as some shaded blocks.

Incredibly, it also has: smileys, card deck symbols(?!), male/female signs, music notes, a sun & some greek/math characters. Interesting bit of history here, this reflects what PCs were used for, or at least _thought to be useful for_ in the 80s: the male/female icons were probably intended to be used for some HR employee database type programs (IBM's bread and butter, after all...), and the card decks for elite hardcore gaming. (More importantly: **are these the first emojis???**)

Implement the `Color` enum and `ColorCode` struct.

The `ColorCode(u8)` single-value struct pattern is interesting, it looks like it's basically like `newtype` in Haskell. `#[repr(transparent)]` ensures that it's represented exactly like the field itself (when would this _not_ happen?).

Also, having to derive `PartialEq` _and_ `Eq` all the time seems like a real wart, surely `Eq` should imply `PartialEq` somehow...?

The `Buffer` struct definition is interesting, I can see why you might need `#[repr(transparent)]` for some more complex types.

Add the `Writer` type definition. How can I make this `pub` if the underlying types aren't `pub`? Does `pub` just mean that I can publicly instantiate the type? Surely not?

The `'static` lifetime makes sense: the VGA buffer is always available.

In `write_byte`, why is `row = BUFFER_HEIGHT - 1`? I would have thought we start at 0, and advance based on character position? Maybe this will be implemented next.

Works though, I get some nice colored text on the bottom of the screen.

Learn about [`volatile`](https://crates.io/crates/volatile). Have a quick peek at the implementation, but it doesn't really make sense -- some advanced Rust magic. Anyway, probably not necessary to actually understand.

Also learn about [`lazy_static`](https://docs.rs/lazy_static/1.0.1/lazy_static/). Feels like a hack, this should really not be necessary... The compiler did give me some helpful-ish-looking solutions that don't rely on this, but this would probably imply messing around with the type definition which I don't want to do.

Next up, [`spin`](https://crates.io/crates/spin), a spinlock. Don't fully understand why we need this for now -- I see why `static mut` would be an issue though.

We've now reduced the unsafe part to `unsafe { &mut *(0xb8000 as *mut Buffer) }`. Bit of a crazy expression: cast the address to a mutable pointer of `Buffer`, dereference it, then take a mutable reference to it.

`Buffer` interprets that memory location as a 2D array of size `BUFFER_WIDTH` * `BUFFER_HEIGHT`. Memory accesses to arrays in Rust are bounds checked by default, so we get memory safety. What happens if I write to `col + 100000` though? I would have expected to get a panic, but I don't seem to. Hmm... Can I even observe a panic yet? Not really, it just loops. Okay.

Instead, set `col + 70`, and the row to `BUFFER_HEIGHT - 5`. Then, if bounds aren't checked, I'd expect the whole string to be written to screen. If we panic, I expect to only see part of it. And I do. Nice!
