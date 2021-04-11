#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragPos;
layout(location = 1) in vec3 fragNorm;

layout(binding = 1) uniform Animation {
    float anim;
};

layout(location = 0) out vec4 outColor;

#include "sky.glsl"

void main() {
    vec3 color = vec3(0.);
    if (fragPos.y > 0.362596) {
        color = vec3(0.055,0.365,0.133);
    } else {
        color = vec3(1.000,0.822,0.642);
    }
    vec3 sun = sun_pos();
    vec3 skyc = sky_color(fragNorm);
    float sun_amt = clamp(dot(sun, fragNorm), 0.0, 1.);
    if (sun.y < 0.) { sun_amt = 0.0; }
    color = skyc * color + sun_amt * color;
    outColor = vec4(color, 1.0);
}
