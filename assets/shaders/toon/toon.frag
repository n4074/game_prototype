#version 330

uniform vec3 lightPos;
uniform vec3 cameraPos;
uniform vec4 objectColour;

in float fragDepth;
in vec3 fragViewNormal;
in vec3 fragWorldNormal;
in vec3 fragPos;

out vec4 outColor;
out float depth;
out vec4 norm;

const float nearPlane = 0.1;
const float farPlane = 100;
const vec4 defaultColor = vec4(1.0);

float diffuse(vec3 lightDir, vec3 norm) {
	return max(dot(norm, lightDir), 0.0);
}

float ambient() {
	return 0.1; 
}

float specular(vec3 lightDir, vec3 norm) {
	float specularStrength = 0.5;
	vec3 viewDir = normalize(cameraPos - fragPos);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
	return specularStrength * spec; 
}

vec4 phong(vec3 lightColor, vec3 lightDir, vec3 norm) {
	float spec = specular(lightDir, norm);
	float diff = diffuse(lightDir, norm);
	float ambient = ambient();
	
	return vec4(vec3(ambient + diff + spec), 1.0);
}

vec4 toon(vec3 lightColor, vec3 lightDir, vec3 norm) {
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
    depth = fragDepth;
    norm = vec4(normalize(fragViewNormal), 0.0);
    
    vec3 lightColor = vec3(1.0);
	vec3 norm = normalize(fragWorldNormal);
	vec3 lightDir = normalize(lightPos);
	
	outColor = toon(lightColor, lightDir, norm) * objectColour;
}