#ifndef INTERFACE_EXPORT
#define INTERFACE_EXPORT
#endif

#include "bindings.h"
#include "pch.h"
#include <stdio.h>

#include "glad/glad.h"
#include "renderer/renderer.h"

EXPORT void c_draw_rect(float x, float y, float width, float height)
{
    sr::srDrawRectangleFilledOutlineC({x, y}, {width, height}, {width / 2.0f, height / 2.0f}, 0.1f);
}