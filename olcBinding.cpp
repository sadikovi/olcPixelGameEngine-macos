#define OLC_PGE_APPLICATION
#include "olcPixelGameEngine.h"
#include "olcBinding.h"

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
  bool OnUserCreate() override
  {
    return onUserCreate(this);
  }

  bool OnUserUpdate(float fElapsedTime) override
  {
    return onUserUpdate(this, fElapsedTime);
  }

  bool OnUserDestroy() override
  {
    return onUserDestroy(this);
  }
};

#ifdef __cplusplus
extern "C" {
#endif

RustBinding binding;

void* create(const char* name) {
  binding.sAppName = name;
  RustBinding* ptr = &binding;
  return (void*) ptr;
}

rcode construct(void* ptr, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync) {
  olc::rcode res = ((RustBinding *) ptr)->Construct(screen_w, screen_h, pixel_w, pixel_h, full_screen, vsync);
  switch (res) {
    case olc::rcode::FAIL: return rcode::FAIL;
    case olc::rcode::OK: return rcode::OK;
    case olc::rcode::NO_FILE: return rcode::NO_FILE;
  }
}

rcode start(void* ptr) {
  olc::rcode res = ((RustBinding *) ptr)->Start();
  switch (res) {
    case olc::rcode::FAIL: return rcode::FAIL;
    case olc::rcode::OK: return rcode::OK;
    case olc::rcode::NO_FILE: return rcode::NO_FILE;
  }
}

#ifdef __cplusplus
}
#endif
