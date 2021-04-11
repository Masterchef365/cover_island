#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragNormal;
layout(location = 1) in vec3 fragRay;

layout(binding = 1) uniform Animation {
    float anim;
};

layout(location = 0) out vec4 outColor;

#include "sky.glsl"

void main() {
    vec3 ref = reflect(normalize(fragRay), normalize(fragNormal));
    ref.y = abs(ref.y);
    vec3 color = sky(ref);
    //vec3 color = normalize(fragRay);
    //vec3 color = fragNormal;
    //vec3 color = ref;
    outColor = vec4(color, 1.0);
}
