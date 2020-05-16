#ifdef __cplusplus
extern "C" {
#endif

// Called once on user create.
bool onUserCreate(void* binding);
// Called for every frame.
bool onUserUpdate(void* binding, float elapsed_time);
// Called once on user destroy.
bool onUserDestroy(void* binding);

enum RCode {
  CONSTRUCT_FAIL,
  CONSTRUCT_NO_FILE,
  START_FAIL,
  START_NO_FILE,
  OK,
};

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Pixel;

// Useful utility functions
int32_t c_rand();

// Starts the main game loop.
// Default values: full_screen = false and vsync = false
RCode start(const char* name, void* binding, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync);

// olcPixelGameEngine API

bool draw(int32_t x, int32_t y, Pixel p);
int32_t screen_width();
int32_t screen_height();


#ifdef __cplusplus
}
#endif
