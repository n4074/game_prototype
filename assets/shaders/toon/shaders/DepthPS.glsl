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

vec3 diffuse(vec3 lightColor, vec3 lightDir, vec3 norm) {
	
	float diff = max(dot(norm, lightDir), 0.0);
	return diff * lightColor;
}

vec3 ambient_(vec3 objectColor) {
	float ambient = 0.1;
	return vec3(ambient); 
}

vec3 specular(vec3 lightColor, vec3 lightDir, vec3 norm) {
	float specularStrength = 0.5;
	vec3 viewDir = normalize(cameraPos - fragPos);
	vec3 reflectDir = reflect(-lightDir, norm);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
	return specularStrength * spec * lightColor; 
}

void main() {
    depth = fragDepth;
    norm = vec4(normalize(fragViewNormal), 0.0);
    
    vec3 lightColor = vec3(1.0);
	vec3 norm = normalize(fragWorldNormal);
	vec3 lightDir = normalize(lightPos - fragPos);
	vec3 spec = specular(lightColor, lightDir, norm);
	vec3 diff = diffuse(lightColor, lightDir, norm);
	vec3 ambient = ambient_(vec3(objectColour));
	

	outColor = vec4(ambient + diff + spec, 1.0) * objectColour;
}