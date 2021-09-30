#version 330

uniform mat4 matVP;
uniform mat4 matGeo;
uniform mat4 matV;
uniform vec4 colorIn;

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 normal;

out vec4 color;
out vec3 fragPos;
out vec3 fragNormal;


void main() {
   // worldspace normal
   
   // this is accurate regardless of scale, but inverse is costly
   fragNormal = mat3(transpose(inverse(matGeo))) * normal;
   //this is inaccurate if we have non-uniform scale
   // see https://learnopengl.com/Lighting/Basic-Lighting
   //fragNormal = vec3(matGeo *  vec4 (normal, 0)); 
   color = colorIn;

   fragPos = vec3(matGeo * vec4(pos, 1.0));
   gl_Position = matVP *  matGeo * vec4(pos, 1);
}
 