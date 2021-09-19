#version 450

layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 1) uniform Transform {
    mat4 Model;
};

layout(location = 0) out vec3 vertexPosition;
layout(location = 1) out vec3 nearPoint;
layout(location = 2) out vec3 farPoint;

vec3 unproject(float x, float y, float z) {
    vec4 unprojectedPoint = inverse(ViewProj) * vec4(x, y, z, 1.0);
    return unprojectedPoint.xyz / unprojectedPoint.w;
}

void main() {
    nearPoint = unproject(Vertex_Position.x, Vertex_Position.y, 0.0);
    farPoint = unproject(Vertex_Position.x, Vertex_Position.y, 1.0);
    vertexPosition = Vertex_Position.xyz;

    // Quads world coords expected to be at -1..1 to fill viewspace range
    gl_Position = vec4(Vertex_Position.xy, 1.0, 1.0);
}