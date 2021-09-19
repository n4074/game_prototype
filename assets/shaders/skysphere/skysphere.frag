#version 450

#define M_PI 3.1415926535897932384626433832795

layout(location = 0) in vec3 vertexPosition;
layout(location = 1) in vec3 nearPoint;
layout(location = 2) in vec3 farPoint;

layout(set = 2, binding = 0) uniform texture2D SkySphere_texture;
layout(set = 2, binding = 1) uniform sampler SkySphere_texture_sampler;

layout(location = 0) out vec4 colour;

void main() {
	vec3 dir = normalize(farPoint - nearPoint);
    float lng = acos(dir.y) / M_PI;
	float lat = atan(dir.x, dir.z) / M_PI / 2.0 + 0.5;

    colour = texture(
        sampler2D(SkySphere_texture, SkySphere_texture_sampler),
        vec2(lat, lng)
    );
}

