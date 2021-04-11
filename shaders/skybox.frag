#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_multiview : require

layout(location = 0) in vec3 fragColor;

layout(binding = 0) uniform CameraUbo {
    mat4 camera[2];
};

layout(binding = 1) uniform Animation {
    float anim;
};

layout(location = 0) out vec4 outColor;

#include "sky.glsl"

void main() {
    vec3 st = mat3(camera[gl_ViewIndex]) * vec3(fragColor.xy, 1.);
    vec3 color = sky(st);
    outColor = vec4(color, 1.0);
}
