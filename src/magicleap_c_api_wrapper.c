#include "ml_graphics.h"
#include "ml_lifecycle.h"

void MLGraphicsFrameInfoInitWrapped(MLGraphicsFrameInfo *frame_info)
{
    MLGraphicsFrameInfoInit(frame_info);
}

void MLGraphicsFrameParamsExInitWrapped(MLGraphicsFrameParamsEx *frame_params)
{
    MLGraphicsFrameParamsExInit(frame_params);
}

void MLLifecycleCallbacksExInitWrapped(MLLifecycleCallbacksEx *inout_callbacks)
{
    MLLifecycleCallbacksExInit(inout_callbacks);
}