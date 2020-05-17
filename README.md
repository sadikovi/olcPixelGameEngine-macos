# olcPixelGameEngine-macos

The repository contains the latest version of https://github.com/OneLoneCoder/olcPixelGameEngine
updated to be built on macos as well as linux and windows.

Supports older macos versions, e.g. builds and runs fine on 10.13 and 10.14.

It also contains Rust bindings which can be used independently of `olcPixelGameEngine` files.

> If the `olcPixelGameEngine.h` file is out of date, then please update it!

## Requirements

Currently X11 is required to run the pixel game engine. X11 is no longer included in macos,
but X11 server and client libraries are available from the XQuartz project
(see https://support.apple.com/en-us/HT201341).

You can download X11 from https://www.xquartz.org/ and easily install it. Typical installation path
is `/usr/X11` and it contains includes and libraries for X11 and OpenGL.

I will be working on XCode and Metal support as well.

## Run the example program

Run this command to build and the example program and then run it:

```shell
g++ -o olcExampleProgram olcExampleProgram.cpp \
  -I/usr/X11/include -L/usr/X11/lib -lX11 -lGL -lpng -lpthread -std=c++17
```

```shell
./olcExampleProgram
```

## Rust bindings

You can write your `olcPixelGameEngine` application in Rust now! This repository contains Rust
bindings to write games in Rust instead of C++, though you can still do the former.
See [examples/](./examples) folder for the examples and API docs.

You can include this repository as a dependency to your project to build and run games in Rust!
