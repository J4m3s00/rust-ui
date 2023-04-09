#ifndef INTERFACE_EXPORT
#define INTERFACE_EXPORT
#endif

#include "bindings.h"
#include "pch.h"
#include <stdio.h>

#include "glad/glad.h"
#include "renderer/renderer.h"

#include "intern.h"

EXPORT void c_draw_rect(float x, float y, float width, float height)
{
    sr::srDrawRectangleFilledOutlineC({x, y}, {width, height}, {0.0f, 0.0f /*width / 2.0f, height / 2.0f*/}, 0.1f);
}
EXPORT void c_draw_circle(float x, float y, float radius)
{
    sr::srDrawCircle({x, y}, radius);
}

EXPORT void c_draw_line(float x1, float y1, float x2, float y2)
{
    sr::srBeginPath(sr::PathType_Stroke);
    sr::srPathLineTo({x1, y1});
    sr::srPathLineTo({x2, y2});
    sr::srEndPath();
}
EXPORT void c_draw_text(float x, float y, const char *text)
{
    sr::srDrawText(get_current_font(), text, {x, y});
}