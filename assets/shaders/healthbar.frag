#version 450

layout(location = 0) out vec4 colour;

layout(set = 3, binding = 2) uniform HealthBar_colour {
    vec4 in_colour;
};

void main() {
    colour = vec4(vec3(1.0), 1.0);
}