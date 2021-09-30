#version 450

uniform sampler2D normalTexture;
uniform vec2 viewPort;

out vec4 outColor;

void main() {
	outColor = ((texture(normalTexture, gl_FragCoord.xy / viewPort.xy)) + 1) / 2;
}