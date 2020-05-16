#define OLC_PGE_APPLICATION
#include "../olcPixelGameEngine.h"
#include "olcRustBindingApp.h"

class RustBindingApp : public olc::PixelGameEngine
{
public:
  void* binding;

public:
  RustBindingApp()
  {
    sAppName = "Rust Binding App";
  }
  ~RustBindingApp()
  {}

public:
  bool inline OnUserCreate() override
  {
    return onUserCreate(this->binding);
  }

  bool inline OnUserUpdate(float fElapsedTime) override
  {
    return onUserUpdate(this->binding, fElapsedTime);
  }

  bool inline OnUserDestroy() override
  {
    return onUserDestroy(this->binding);
  }
};

#ifdef __cplusplus
extern "C" {
#endif

// C++ rand utility function
int32_t c_rand() {
  return rand();
}

// Should be available for the duration of the application.
RustBindingApp app;

RCode start(const char* name, void* binding, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync) {
  app.sAppName = name;
  app.binding = binding;

  olc::rcode res;

  res = app.Construct(screen_w, screen_h, pixel_w, pixel_h, full_screen, vsync);
  switch (res) {
    case olc::rcode::FAIL: return RCode::CONSTRUCT_FAIL;
    case olc::rcode::NO_FILE: return RCode::CONSTRUCT_NO_FILE;
    case olc::rcode::OK: break;
  }

  res = app.Start();
  switch (res) {
    case olc::rcode::FAIL: return RCode::START_FAIL;
    case olc::rcode::NO_FILE: return RCode::START_NO_FILE;
    case olc::rcode::OK: break;
  }

  return RCode::OK;
}

#define OLC_PIXEL(p) (olc::Pixel(p.r, p.g, p.b, p.a))

bool draw(int32_t x, int32_t y, Pixel p) {
  return app.Draw(x, y, OLC_PIXEL(p));
}

int32_t screen_width() {
  return app.ScreenWidth();
}

int32_t screen_height() {
  return app.ScreenHeight();
}

#ifdef __cplusplus
}
#endif
