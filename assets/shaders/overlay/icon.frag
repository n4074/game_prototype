#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 1) uniform Overlay_icon_colour {
    vec4 Color;
};

layout(set = 2, binding = 2) uniform texture2D Overlay_icon_texture;
layout(set = 2, binding = 3) uniform sampler Overlay_icon_texture_sampler;

void main() {
    vec4 color = Color;
    color *= texture(
        sampler2D(Overlay_icon_texture, Overlay_icon_texture_sampler),
        v_Uv);
    o_Target = color;
}