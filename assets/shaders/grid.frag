#version 450

#define M_PI 3.1415926535897932384626433832795

layout(location = 0) in vec3 vertexPosition;
layout(location = 1) in vec3 nearPoint;
layout(location = 2) in vec3 farPoint;

layout(location = 0) out vec4 colour;


layout(set = 2, binding = 0) uniform MyMaterialX_color {
    vec4 color;
};
//float computeDepth(vec3 pos) {
//	vec4 clip_space_pos = iProjection * iModelView * vec4(pos.xyz, 1.0);
//	float clip_space_depth = clip_space_pos.z / clip_space_pos.w;
//
//	float far = gl_DepthRange.far;
//	float near = gl_DepthRange.near;
//
//	float depth = (((far-near) * clip_space_depth) + near + far) / 2.0;
//
//	return depth;
//}

void main() {
    //vec3 fragPos3D = nearPoint + t * (farPoint - nearPoint);

    //gl_FragDepth = computeDepth(fragPos3D);

    //float linearDepth = computeLinearDepth(fragPos3D);
    //float fading = max(0, (0.5 - linearDepth));

    //o_Colour = (grid(fragPos3D, 1, true) + grid(fragPos3D, 0.1, true) * float(t > 0)); // adding multiple resolution for the grid

    float t = -nearPoint.y / (farPoint.y-nearPoint.y);
    vec3 groundPoint = nearPoint + t * (farPoint-nearPoint);

    //float c = (
    //    int(abs(round(groundPoint.x)) + abs(round(groundPoint.z)))
    //) % 2;

    float c = float(abs(((groundPoint.x * groundPoint.x) + (groundPoint.z * groundPoint.z)) - M_PI*M_PI) < 0.05);

    colour = vec4(vec3(c), float(c > 0 && t > 0));

    gl_FragDepth = 0.0;

}

