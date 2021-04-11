#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragPos;
layout(location = 1) in vec3 fragNorm;

layout(binding = 1) uniform Animation {
    float anim;
};

layout(location = 0) out vec4 outColor;

void main() {
    vec3 color = vec3(0.);
    if (fragPos.y > 0.362596) {
        color = vec3(0.055,0.365,0.133);
    } else {
        color = vec3(1.000,0.822,0.642);
    }
    color = fragNorm;
    outColor = vec4(color, 1.0);
}
