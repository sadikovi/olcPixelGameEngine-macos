brew install sdl2

brew install sdl2_image

Compile OLC pixel game engine on macos:
Install X11 from https://support.apple.com/en-us/HT201341 or https://www.xquartz.org/.

```shell
g++ -o olcExampleProgram olcExampleProgram.cpp \
  -I/usr/X11/include -L/usr/X11/lib -lX11 -lGL -lpng -lpthread -std=c++17
```
