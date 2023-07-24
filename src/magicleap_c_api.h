//Derived from: https://github.com/servo/webxr
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#define VK_VERSION_1_0 1

#include "vk_platform.h"
#include "vulkan.h"

#include "ml_api.h"
#include "ml_graphics.h"
#include "ml_head_tracking.h"
#include "ml_lifecycle.h"
#include "ml_logging.h"
#include "ml_perception.h"
#include "ml_snapshot.h"

#include "magicleap_c_api_wrapper.h"
#include "misc.h"