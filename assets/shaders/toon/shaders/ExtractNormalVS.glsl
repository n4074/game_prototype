#version 330

uniform mat4 matVP;
uniform mat4 matGeo;

layout (location = 0) in vec4 pos;

void main() {
   gl_Position = pos;
}
 