#ifdef __cplusplus
extern "C" {
#endif

enum rcode {
  FAIL = 0,
  OK = 1,
  NO_FILE = -1
};

bool onUserCreate(void* ptr);
bool onUserUpdate(void* ptr, float fElapsedTime);
bool onUserDestroy(void* ptr);

void* create(const char* name);
// Default values: full_screen = false and vsync = false
rcode construct(void* ptr, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync);
rcode start(void* ptr);

#ifdef __cplusplus
}
#endif
