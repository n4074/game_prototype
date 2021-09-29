#version 330

uniform mat4 matVP;
uniform mat4 matGeo;
uniform mat4 matV;
uniform vec4 colorIn;

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;

out float fragDepth;
out vec3 fragViewNormal;
out vec3 fragWorldNormal;
out vec3 fragPos;

const float nearPlane = -0.1;
const float farPlane = -100.0;

void main() {
   gl_Position = matVP *  matGeo * vec4(pos, 1);
   fragViewNormal = vec3(matV * matGeo * vec4(normal, 0.0));
   fragWorldNormal = vec3(matGeo * vec4(normal, 0.0));
   fragPos = vec3(matGeo * vec4(pos, 1.0));
   vec4 viewPos = matV * matGeo * vec4(pos, 1);
   fragDepth = (viewPos.z - nearPlane) / (farPlane - nearPlane);
   
}
 
