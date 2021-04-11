
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

layout(location = 0) out vec3 fragNormal;
layout(location = 1) out vec3 fragRay;

void main() {
    vec3 pos = inPosition;
    float angle = pos.x + pos.z + anim;

    //const float mult = 1. / 5;
    const float mult = 0.;
    float y = cos(angle) * mult;
    pos = vec3(pos.x, y, pos.z);
    vec4 pos_world = model * vec4(pos, 1.0);
    gl_Position = camera[gl_ViewIndex] * pos_world;

    fragNormal = vec3(-sin(angle) * mult, 1., -sin(angle) * mult) / 5.;
    mat3 inv_cam = inverse(mat3(camera[gl_ViewIndex]));
    fragRay = inv_cam * gl_Position.xyz;
}

