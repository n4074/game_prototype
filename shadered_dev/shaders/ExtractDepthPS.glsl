#version 450

uniform sampler2D depthTexture;
uniform vec2 viewPort;

const float nearPlane = 0.1;
const float farPlane = 100;

out vec4 outColor;

void main() {
	outColor  = texture(depthTexture, gl_FragCoord.xy / viewPort.xy);
}