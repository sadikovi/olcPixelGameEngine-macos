#ifdef __cplusplus
extern "C" {
#endif

enum RCode {
  CONSTRUCT_FAIL,
  CONSTRUCT_NO_FILE,
  START_FAIL,
  START_NO_FILE,
  OK,
};

// Called once on user create.
bool onUserCreate(void* app, void* binding);
// Called for every frame.
bool onUserUpdate(void* app, void* binding, float elapsed_time);
// Called once on user destroy.
bool onUserDestroy(void* app, void* binding);

// Returns pointer to the RustBindingApp.
void* create();
// Starts the main game loop.
// Default values: full_screen = false and vsync = false
RCode start(const char* name, void* binding, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync);

#ifdef __cplusplus
}
#endif
