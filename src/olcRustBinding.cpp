#define OLC_PGE_APPLICATION
#include "../olcPixelGameEngine.h"
#include "olcRustBinding.h"

class RustBinding : public olc::PixelGameEngine
{
public:
  RustBinding()
  {
    sAppName = "Rust Binding";
  }
  ~RustBinding()
  {}

public:
  bool inline OnUserCreate() override
  {
    return onUserCreate(this);
  }

  bool inline OnUserUpdate(float fElapsedTime) override
  {
    return onUserUpdate(this, fElapsedTime);
  }

  bool inline OnUserDestroy() override
  {
    return onUserDestroy(this);
  }
};

#ifdef __cplusplus
extern "C" {
#endif

// Should be available for the duration of the application.
RustBinding binding;

void* create(const char* name) {
  binding.sAppName = name;
  RustBinding* ptr = &binding;
  return (void*) ptr;
}

RCode construct(void* ptr, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync) {
  olc::rcode res = ((RustBinding *) ptr)->Construct(screen_w, screen_h, pixel_w, pixel_h, full_screen, vsync);
  switch (res) {
    case olc::rcode::FAIL: return RCode::FAIL;
    case olc::rcode::OK: return RCode::OK;
    case olc::rcode::NO_FILE: return RCode::NO_FILE;
  }
}

RCode start(void* ptr) {
  olc::rcode res = ((RustBinding *) ptr)->Start();
  switch (res) {
    case olc::rcode::FAIL: return RCode::FAIL;
    case olc::rcode::OK: return RCode::OK;
    case olc::rcode::NO_FILE: return RCode::NO_FILE;
  }
}

#ifdef __cplusplus
}
#endif
