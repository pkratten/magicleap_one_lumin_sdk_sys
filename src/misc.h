#include <stdint.h>

typedef struct ANativeWindow ANativeWindow;

int32_t
ANativeWindow_setBuffersGeometry(ANativeWindow *window,
                                 int32_t width, int32_t height, int32_t format);