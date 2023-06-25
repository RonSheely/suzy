/* SPDX-License-Identifier: (MIT OR Apache-2.0 OR Zlib) */
/* Copyright © 2021 Violet Leonard */

#version 100

uniform mat4 TRANSFORM;

attribute highp vec2 in_xy;
attribute mediump vec2 in_uv;
attribute lowp vec4 in_color;
attribute mediump vec4 in_config;
attribute mediump float in_smoothing;

varying lowp vec4 pass_color;
varying mediump vec2 pass_config;
varying mediump vec2 pass_uv;
varying lowp vec2 pass_distance;
varying mediump float pass_smoothing;

void main() {
    lowp float dy = float(in_config.z > 0.5);
    lowp float dx = float(max(in_config.z - 0.5 * dy, 0.0) > 0.25);

    gl_Position = TRANSFORM * vec4(in_xy, 0, 1);
    pass_color = in_color;
    pass_config = in_config.xy;
    pass_uv = in_uv;
    pass_distance = vec2(dx, dy);
    pass_smoothing = in_smoothing;
}
