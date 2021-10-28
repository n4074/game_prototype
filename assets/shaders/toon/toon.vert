#version 450

//uniform mat4 matVP;
//uniform mat4 matGeo;
//uniform mat4 matV;
//uniform vec4 colorIn;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 matVP;
};

layout(set = 0, binding = 2) uniform CameraView {
    mat4 matV;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 matGeo;
};

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;

layout(location = 0) out float fragDepth;
layout(location = 1) out vec4 fragViewNormal;
layout(location = 2) out vec4 fragWorldNormal;
layout(location = 3) out vec3 fragPos;

const float nearPlane = -1.0;
const float farPlane = -1000.0;

void main() {
   gl_Position = matVP *  matGeo * vec4(Vertex_Position, 1);
   fragViewNormal = matV * matGeo * vec4(Vertex_Normal, 0.0);
   fragWorldNormal = matGeo * vec4(Vertex_Normal, 0.0);
   fragPos = vec3(matGeo * vec4(Vertex_Position, 1.0));
   vec4 viewPos = matV * matGeo * vec4(Vertex_Position, 1);
   fragDepth = (viewPos.z - nearPlane) / (farPlane - nearPlane);
}
 
