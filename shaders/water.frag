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
    //ref.y = abs(ref.y);
    //vec3 color = normalize(fragRay);
    //vec3 color = normalize(fragNormal);
    //vec3 color = ref;
    vec3 normal = normalize(fragNormal);
    vec3 ray = normalize(fragRay);
    vec3 ref = reflect(ray, normal);
    vec3 color = mix(vec3(0., 0., 1.), sky(ref), 0.9);
    outColor = vec4(color, 1.0);
}
