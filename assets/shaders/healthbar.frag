#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 colour;

layout(set = 3, binding = 0) uniform HealthBar_fill {
    float fill;
};

void main() {
    colour = vec4(0.0);
    float green = step(v_Uv.x, fill);
    float red = 1.0 - green;
    colour = vec4(red, green, 0.0, 1.0);
}