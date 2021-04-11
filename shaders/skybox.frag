#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;

layout(binding = 1) uniform Animation {
    float anim;
};

layout(location = 0) out vec4 outColor;

void main() {
    vec3 st = fragColor;
    vec3 color = mix(
        vec3(0.835,0.476,0.831), 
    	vec3(0.000,0.468,1.000), 
        (st.y + -0.408) / 0.464
    );
    outColor = vec4(color, 1.0);
}
