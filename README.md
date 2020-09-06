# omni-rs

An engine written in rust with the goal of one codebase for multiple platforms.

## Goals
- Users should only need to write the majority of code once
- Support for multiple platforms:
    - Desktop
        - Linux :heavy_check_mark:
        - Windows
        - MacOS
    - Android
    - iOS
    - potentially more (create issues/pull requests)
- Users should only have to write in rust

## How it works
The entrypoint to the program depends on the platform:
- Desktop: `desktop/src/main.rs`
- Android: 
- iOS:

The entrypoint then calls into the `src/platform/` specific init function which initializes
the engine. Then the platform gives callbacks to functions written in the platform's own
language (eg. Android = Java). These callbacks are the implementations of platform specific
code like graphics, sound, input, etc.

Then the entrypoint controls the game loop and calls into the app for functions like
input(), update(), and render.

To accomplish communication between languages [FFI](https://doc.rust-lang.org/nomicon/ffi.html) is used.

## Why do it this way?
In all the reading that I have done on the subject of writing programs for android/iOS with rust/C I've
found that you must start the program in the platform's language. (Android=Java/Kotlin, iOS=Swift).

So, this is the simplest way I could think up to get this all working.

If you know of a simpler way this could be done, please submit an issue, and we can discuss it.