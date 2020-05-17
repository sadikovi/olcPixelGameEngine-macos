# olcPixelGameEngine-macos

The repository contains the latest version of https://github.com/OneLoneCoder/olcPixelGameEngine
updated to be built on macOS.

Supports older macOS versions, e.g. builds and runs fine on OSX 10.13 and OSX 10.14.
No XCode required.

> If the `olcPixelGameEngine.h` file is out of date, then feel free to open a PR to update it!

## Requirements

The only requirement is having X11 installed. X11 is no longer included in macOS, but X11 server
and client libraries are available from the XQuartz project (see https://support.apple.com/en-us/HT201341).

You can download X11 from https://www.xquartz.org/ and easily install it. Typical installation
path is `/usr/X11` and it contains includes and libraries for X11 and OpenGL.

I will be working on XCode and Metal support as well.

## Build and run the example program

Run this command to build and the example program and then run it:

```shell
g++ -o olcExampleProgram olcExampleProgram.cpp \
  -I/usr/X11/include -L/usr/X11/lib -lX11 -lGL -lpng -lpthread -std=c++17
```

```shell
./olcExampleProgram
```
