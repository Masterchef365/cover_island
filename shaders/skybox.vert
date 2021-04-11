
#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_multiview : require

layout(binding = 0) uniform CameraUbo {
    mat4 camera[2];
};

layout(binding = 1) uniform Animation {
    float anim;
};

layout(push_constant) uniform Model {
    mat4 model;
};

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out mat3 camInv;
layout(location = 3) out vec3 fragPos;

void main() {
    gl_Position = vec4(inPosition.xy, 0.9998, 1.);
    camInv = inverse(mat3(camera[gl_ViewIndex]));
    fragPos = inPosition;
}

