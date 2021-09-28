#version 330

uniform vec3 lightPos;
uniform vec3 cameraPos;

in vec3 fragNormal;
in float fragDepth;

out vec4 outColor;
out float depth;
out vec4 norm;

const float nearPlane = 0.1;
const float farPlane = 100;

void main() {
    outColor = vec4(1.0, 0.0, 0.0, 0.0);
    depth = fragDepth;
    norm = vec4(normalize(fragNormal), 0.0);
}