#version 450
#define identity mat3(0, 0, 0, 0, 1, 0, 0, 0, 0)
#define edge0 mat3(1, 0, -1, 0, 0, 0, -1, 0, 1)
#define edge1 mat3(0, 1, 0, 1, -4, 1, 0, 1, 0)
#define edge2 mat3(-1, -1, -1, -1, 8, -1, -1, -1, -1)
#define sharpen mat3(0, -1, 0, -1, 5, -1, 0, -1, 0)
#define box_blur mat3(1, 1, 1, 1, 1, 1, 1, 1, 1) * 0.1111
#define gaussian_blur mat3(1, 2, 1, 2, 4, 2, 1, 2, 1) * 0.0625
#define emboss mat3(-2, -1, 0, -1, 1, 1, 0, 1, 2)

uniform sampler2D depthTexture;
uniform sampler2D normalTexture;

uniform vec2 viewPort;
out vec4 outColor;

// Find coordinate of matrix element from index
vec2 kpos(int index)
{
    return vec2[9] (
        vec2(-1, -1), vec2(0, -1), vec2(1, -1),
        vec2(-1, 0), vec2(0, 0), vec2(1, 0), 
        vec2(-1, 1), vec2(0, 1), vec2(1, 1)
    )[index] / viewPort.xy;
}

vec2 neighbour(int index)
{
    return vec2[8] (
        vec2(-1, -1), vec2(0, -1), vec2(1, -1),
        vec2(-1, 0), vec2(1, 0), 
        vec2(-1, 1), vec2(0, 1), vec2(1, 1)
    )[index] / viewPort.xy;
}

// Extract region of dimension 3x3 from sampler centered in uv
// sampler : texture sampler
// uv : current coordinates on sampler
// return : an array of mat3, each index corresponding with a color channel
mat3[3] region3x3(sampler2D sampler, vec2 uv)
{
    // Create each pixels for region
    vec4[9] region;
    
    for (int i = 0; i < 9; i++)
        region[i] = texture(sampler, uv + kpos(i));

    // Create 3x3 region with 3 color channels (red, green, blue)
    mat3[3] mRegion;
    
    for (int i = 0; i < 3; i++)
        mRegion[i] = mat3(
        	region[0][i], region[1][i], region[2][i],
        	region[3][i], region[4][i], region[5][i],
        	region[6][i], region[7][i], region[8][i]
    	);
    
    return mRegion;
}

// Convolve a texture with kernel
// kernel : kernel used for convolution
// sampler : texture sampler
// uv : current coordinates on sampler
vec4 convolution(mat3 kernel, sampler2D sampler, vec2 uv)
{
    vec4 fragment;
    float threshold = 0.07;
    
    // Extract a 3x3 region centered in uv
    mat3[3] region = region3x3(sampler, uv);
    
    // for each color channel of region
    for (int i = 0; i < 3; i++)
    {
        // get region channel
        mat3 rc = region[i];
        // component wise multiplication of kernel by region channel
        mat3 c = matrixCompMult(kernel, rc);
        // add each component of matrix
        float r = c[0][0] + c[1][0] + c[2][0]
                + c[0][1] + c[1][1] + c[2][1]
                + c[0][2] + c[1][2] + c[2][2];
        
        // for fragment at channel i, set result
        
        fragment[i] = r;
    }
    
    return fragment;    
}

const vec2[8] neighbors = vec2[8](
    vec2(-1, -1), vec2(0, -1), vec2(1, -1),
    vec2(-1, 0), vec2(1, 0), 
    vec2(-1, 1), vec2(0, 1), vec2(1, 1)
);

float max_angle(sampler2D sampler, vec2 uv) {
	float angle = 1.0;
	float ave = 0.0;
	
	vec3 center = normalize(texture(sampler, uv).xyz);
	
	for (int i = 0; i < 8; i++) {
		vec3 neighbor = normalize(texture(sampler, uv + neighbour(i)).xyz);
		float local_angle = abs(dot(center, neighbor));
		ave += local_angle;
		angle = max(angle, local_angle);
	}
	return ave / 8;
}

const float threshold = 0.99;
void main() {
	vec2 uv = gl_FragCoord.xy / viewPort.xy;
	vec2 offset = vec2(1, 1) / viewPort.xy;
	vec3 center = normalize(texture(normalTexture, uv).xyz);
	vec3 n = normalize(texture(normalTexture, uv + (0, offset.y)).xyz);
	vec3 s = normalize(texture(normalTexture, uv + (0, - offset.y)).xyz);
	vec3 e = normalize(texture(normalTexture, uv + (offset.x, 0)).xyz);
	vec3 w = normalize(texture(normalTexture, uv + (- offset.x, 0)).xyz);
	
	float n_dot = abs(dot(center, n));
	float s_dot = abs(dot(center, s));
	float e_dot = abs(dot(center, e));
	float w_dot = abs(dot(center, w));
	//vec4 color = convolution(edge2, depthTexture, gl_FragCoord.xy / viewPort.xy);
	//float maxAngle = maximum_dotproduct(normalTexture, uv);
	//float angle = min(min(min(n_dot, s_dot), e_dot), w_dot);
	float angle = max_angle(normalTexture, uv);
	//float line = float(maxAngle < 0.05);
	
	if (angle < threshold) {
		outColor = vec4(0.0, 1.0, 0.0,1.0);
	} else {
		outColor = vec4(0.0);
	}
	vec4 conv = convolution(edge2, depthTexture, uv);
	float lum = max(max(conv.x,conv.y), conv.z);
	outColor = vec4(vec3(lum), 1.0);

}
 