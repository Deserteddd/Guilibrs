#version 450

// Get the vertex position from the vertex buffer
layout (location = 0) in vec3 pos;

// Output a color to the fragment shader
layout (location = 0) out vec3 frag_color;

// Uniforms that are pushed via SDL_PushGPUVertexUniformData
layout(set = 1, binding = 0) uniform PushConstants {
    float rotationX;
    float rotationY;
};

void main(void) {
	gl_Position = vec4(pos, 1.0);

    // Create a frag color based on the vertex position
    frag_color = normalize(pos) * 0.5 + 0.5;
}

