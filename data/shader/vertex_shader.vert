#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 tex_coord;

layout(location = 0) out vec2 o_tex_coord;
layout(location = 1) out vec3 o_normal;
layout(location = 2) out vec3 frag_position;

//Global uniforms
layout(set = 0, binding = 0) uniform TransformationData {
  mat4 model;
  mat4 view;
  mat4 proj;
} u_main;

void main() {
  gl_Position = u_main.proj * u_main.view * u_main.model * vec4(position, 1.0);

  o_tex_coord = tex_coord;

  o_normal = mat3(transpose(inverse(u_main.model))) * normal;
  frag_position = vec3(u_main.model * vec4(position, 1.0));
}
