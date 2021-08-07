#version 450

layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 1) uniform Transform {
    mat4 Model;
};

//layout(location = 0) out vec3 vertexPosition;
//layout(location = 1) out vec3 nearPoint;
//layout(location = 2) out vec3 farPoint;

layout(location = 0) out float near; //0.01
layout(location = 1) out float far; //100
layout(location = 2) out vec3 nearPoint;
layout(location = 3) out vec3 farPoint;
layout(location = 4) out mat4 fragView;
layout(location = 8) out mat4 fragProj;

vec3 gridPlane[4] = vec3[](
    vec3(-1, -1, 0), vec3(-1, 1, 0),
    vec3(1, 1, 0), vec3(1, -1, 0)
);

vec3 UnprojectPoint(float x, float y, float z, mat4 ViewProj) {
    //mat4 viewInv = inverse(view);
    //mat4 projInv = inverse(projection);
    vec4 unprojectedPoint =  inverse(ViewProj) * vec4(x, y, z, 1.0);
    return unprojectedPoint.xyz / unprojectedPoint.w;
}

void main() {
    //vertexPosition = Vertex_Position;
    //gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    vec3 p = gridPlane[gl_VertexIndex].xyz;
    nearPoint = UnprojectPoint(p.x, p.y, 0.0, ViewProj).xyz; // unprojecting on the near plane
    farPoint = UnprojectPoint(p.x, p.y, 1.0, ViewProj).xyz; // unprojecting on the far plane
    near = 1.0;
    far = 50;
    fragView = ViewProj;
    gl_Position = vec4(p, 1.0); // using directly the clipped coordinates
    //gl_Position = ViewProj * vec4(gridPlane[gl_VertexIndex].xyz, 1.0);
}