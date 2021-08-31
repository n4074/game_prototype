#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;


layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 0, binding = 2) uniform CameraView {
    mat4 View;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) uniform Billboard_offset {
    vec3 offset;
};

void main() {
    mat4 Proj = ViewProj * View; // not sure why View is already inverse
    mat4 myModel = mat4(
        Model[0][0], 0.0, 0.0, Model[0][3],
        0.0, Model[1][1], 0.0, Model[1][3],
        0.0, 0.0, Model[2][2], Model[2][3],
        0.0, 0.0, 0.0, 1.0);

    //gl_Position = Proj * 
    //    (inverse(View) * Model * vec4(0.0, 0.0, 0.0, 1.0) 
    //        + vec4(Vertex_Position.x,Vertex_Position.y, 0.0, 0.0) + vec4(offset, 0.0));

    gl_Position = Proj * 
        (inverse(View) * Model * vec4(0.0, 0.0, 0.0, 1.0) +
            (myModel * vec4(Vertex_Position.x,Vertex_Position.y, 0.0, 1.0)) + vec4(offset, 0.0));

    v_Uv = Vertex_Uv;
}