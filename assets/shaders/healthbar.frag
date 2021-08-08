#version 450

layout(location = 0) out vec4 colour;

layout(set = 0, binding = 0) uniform HealthBar_colour {
    vec4 colour;
};

layout(set = 0, binding = 0) uniform HealthBar_offset {
    vec3 offset;
};

void main() {
    colour = vec4(vec3(1.0), 0.5);
}

