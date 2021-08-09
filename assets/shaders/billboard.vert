#version 450

layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 0, binding = 2) uniform CameraView {
    mat4 View;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 2) uniform HealthBar_colour {
    vec4 colour;
};

layout(set = 2, binding = 0) uniform HealthBar_offset {
    vec3 offset;
};

void main() {
    mat4 Proj = ViewProj * View; // not sure why View is already inverse

    gl_Position = Proj * 
        (inverse(View) * Model * vec4(0.0, 0.0, 0.0, 1.0) 
            + vec4(Vertex_Position.x,Vertex_Position.y, 0.0, 0.0) + vec4(offset, 0.0));

}