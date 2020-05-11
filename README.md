# olcPixelGameEngine-macos

The repository contains the latest version of https://github.com/OneLoneCoder/olcPixelGameEngine
and scripts to build the file with CLI on macos.

If the `olcPixelGameEngine.h` file is out of date, then please update it!

## Requirements

X11 is required to run the pixel game engine. X11 is no longer included in macos, but X11 server and
client libraries are available from the XQuartz project (see https://support.apple.com/en-us/HT201341).

You can download X11 from https://www.xquartz.org/ and easily install it. Typical installation path
is `/usr/X11` and it contains includes and libraries for X11 and OpenGL.

I will be working on XCode and Metal support, see TODOs for more information.

## Run the example program

Run this command to build and the example program and then run it:

```shell
g++ -o olcExampleProgram olcExampleProgram.cpp \
  -I/usr/X11/include -L/usr/X11/lib -lX11 -lGL -lpng -lpthread -std=c++17
```

```shell
./olcExampleProgram
```

## TODOs
- :white_check_mark: Make it work with X11 and OpenGL.
- TODO Make it work with XCode and Metal.

## Rust bindings

This repository also contains Rust bindings to write games in Rust instead of C++.
See [build.rs](./build.rs) and source files for more details. Also, see examples folder for more
information and available API.

You can include this repository as a dependency to your project to build and run games!
