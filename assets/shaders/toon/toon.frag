#version 460

//uniform vec3 lightPos;
//uniform vec3 cameraPos;
//uniform vec4 objectColour;

layout(set = 0, binding = 1) uniform CameraPosition {
    vec3 cameraPos;
};

layout(set = 2, binding = 0) uniform ColorMaterial_color {
    vec4 objectColour;
};


const vec4 lightPos = vec4(0.0, 500.0, -1000.0, 1.0);
//const vec3 cameraPos = vec3(0.0);
//const vec4 objectColour = vec4(1.0, 0.0, 1.0, 1.0);

layout(location = 0) in float fragDepth;
layout(location = 1) in vec4 fragViewNormal;
layout(location = 2) in vec4 fragWorldNormal;
layout(location = 3) in vec3 fragPos;

layout(location = 0) out vec4 outColor;
//layout(location = 1) out float depth;
//layout(location = 2) out vec4 norm;

const float nearPlane = 0.1;
const float farPlane = 100;
const vec4 defaultColor = vec4(1.0);

float diffuse(vec4 lightDir, vec4 norm) {
	return max(dot(norm, lightDir), 0.0);
}

float ambient() {
	return 0.1; 
}

float specular(vec4 lightDir, vec4 norm) {
	float specularStrength = 0.5;
	vec3 viewDir = normalize(cameraPos - fragPos);
	vec3 reflectDir = reflect(vec3(-lightDir), vec3(norm));
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
	return specularStrength * spec; 
}

vec4 phong(vec3 lightColor, vec4 lightDir, vec4 norm) {
	float spec = specular(lightDir, norm);
	float diff = diffuse(lightDir, norm);
	float ambient = ambient();
	
	return vec4(vec3(ambient + diff + spec), 1.0);
}

vec4 toon(vec3 lightColor, vec4 lightDir, vec4 norm) {
	float specThreshold = 0.01;
	float ambient = ambient();
	float spec = specular(lightDir, norm);
	float diff = diffuse(lightDir, norm);

	float intensity = diff;
	if (spec > specThreshold) {
		intensity = 1.0;
	} else if (intensity > 0.75) {
		intensity = 0.95;
	} else if (intensity > 0.5) {
		intensity = 0.8;
 	} else if (intensity > 0.1) {
 		intensity = 0.6;
 	} else {
 		intensity = 0.1;
	}
	return vec4(vec3(intensity), 1.0);
}

void main() {
    //depth = fragDepth;
    //norm = vec4(normalize(fragViewNormal), 0.0);
    
    vec3 lightColor = vec3(1.0);
	vec4 norm = normalize(fragWorldNormal);
	vec4 lightDir = normalize(lightPos);
	
	outColor = toon(lightColor, lightDir, norm) * objectColour;

}