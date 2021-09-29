#version 450
#define identity mat3(0, 0, 0, 0, 1, 0, 0, 0, 0)
#define edge0 mat3(1, 0, -1, 0, 0, 0, -1, 0, 1)
#define edge1 mat3(0, 1, 0, 1, -4, 1, 0, 1, 0)
#define edge2 mat3(-1, -1, -1, -1, 8, -1, -1, -1, -1)
#define sharpen mat3(0, -1, 0, -1, 5, -1, 0, -1, 0)
#define box_blur mat3(1, 1, 1, 1, 1, 1, 1, 1, 1) * 0.1111
#define gaussian_blur mat3(1, 2, 1, 2, 4, 2, 1, 2, 1) * 0.0625
#define emboss mat3(-2, -1, 0, -1, 1, 1, 0, 1, 2)

#define sobel_gx mat3(1,0,-1,2,0,-2,1,0,-1)
#define sobel_gy mat3(1,2,1,0,0,0,-1,-2,-1)

uniform sampler2D depthTexture;
uniform sampler2D normalTexture;
uniform sampler2D screenTexture;

uniform vec2 viewPort;
out vec4 outColor;
vec2 neighbours(int index)
{
    return vec2[9] (
        vec2(-1, -1), vec2(0, -1), vec2(1, -1),
        vec2(-1, 0), vec2(0, 0), vec2(1, 0), 
        vec2(-1, 1), vec2(0, 1), vec2(1, 1)
    )[index] / viewPort.xy;
}


vec2 neighbour_wo(int index)
{
    return vec2[8] (
        vec2(-1, -1), vec2(0, -1), vec2(1, -1),
        vec2(-1, 0), vec2(1, 0), 
        vec2(-1, 1), vec2(0, 1), vec2(1, 1)
    )[index] / viewPort.xy;
}

mat3[3] region3x3(sampler2D sampler, vec2 uv)
{
    // Create each pixels for region
    vec4[9] region;
    
    for (int i = 0; i < 9; i++) {
        region[i] = texture(sampler, uv + neighbours(i));
	}

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
        
        fragment[i] = r;
    }
    
    return fragment;    
}

float normalDifference(sampler2D sampler, vec2 uv)
{
    float[9] norms;
    
    float normDiff = 0.0;
    
    vec3 center = vec3(texture(sampler, uv));
    
    for (int i = 0; i < 8; i++) {
        vec3 texel = vec3(texture(sampler, uv + neighbour_wo(i)));
        
        normDiff += distance(center, texel);
	}
    
    return normDiff;    
}

vec3 sobel(sampler2D sampler, vec2 uv) {
	vec3 gx = vec3(convolution(sobel_gx, sampler, uv));
	vec3 gy = vec3(convolution(sobel_gy, sampler, uv));
	
	return gx * gx + gy * gy;
}

vec4 convolutionDotProduct(mat3 kernel, sampler2D sampler, vec2 uv)
{
    vec4 fragment;
    float threshold = 0.07;
    
    // Extract a 3x3 region centered in uv
    mat3[3] region = region3x3(sampler, uv);
    
    // for each color channel of region
    for (int i = 0; i < 3; i++)
    {
        // get region channel
        mat3 c = region[i];
        // component wise multiplication of kernel by region channel
       //mat3 c = matrixCompMult(kernel, rc);
        // add each component of matrix
        float r = c[0][0] + c[1][0] + c[2][0]
                + c[0][1] + c[1][1] + c[2][1]
                + c[0][2] + c[1][2] + c[2][2];
        
        fragment[i] = r;
    }
    
    return fragment;    
}


vec2 screen_uv() {
	return gl_FragCoord.xy / viewPort.xy;
}

const float threshold = 0.001;
void main() {
	vec2 uv = screen_uv();
	
	vec4 normC = convolution(edge2, normalTexture, uv);
	vec4 depthC = convolution(edge2, depthTexture, uv);
	
	
	float normX = abs(normC.x);
	float normY = abs(normC.y);
	float normZ = abs(normC.z);
	float average = (normC.x + normC.y + normC.z) / 1;
	float dropZ = normC.x + normC.y;
	float normalDiff_ = normalDifference(normalTexture, uv);
	
	
	float depth = abs(depthC.x);
	//depth = float(depth > threshold);
	vec3 sobel = sobel(normalTexture, uv);
	float[1] one = float[1](1);

	vec3[10] samples = vec3[10] (
		vec3(step(0.9, normalDiff_)),
		vec3(mix(smoothstep(0.0, 0.01, depth), normalDiff_, 0.2)),
		vec3(average),
		vec3(sobel),
		vec3(length(sobel)),
		vec3(dropZ),
		vec3(length(normC.y)),
		vec3(length(normC)),
		vec3(mix(smoothstep(0.0, 0.01, depth), length(normC), 0.2)),
		vec3(smoothstep(0.0, 0.5, sobel))
	);
	
	vec3 final = samples[2];
	
	//gl_FragDepth = 0.01;
	
	float lineThreshold = 0.01;
	
	vec4 screen = texture(screenTexture, uv);
	
	if (max(max(final.x, final.y), final.z) > lineThreshold) {
		outColor = vec4(vec3(step(length(final), lineThreshold)), 1.0);
		//outColor = vec4(final, 1.0);
	} else {
		outColor = screen;
	}
}
 