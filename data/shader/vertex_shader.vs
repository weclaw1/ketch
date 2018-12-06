#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 o_color;

//Global uniforms
layout(set = 0, binding = 0) uniform TransformationData {
  mat4 model;
  mat4 view;
  mat4 proj;
} u_main;

void main() {
  //The proj has been manipulated like here: https://matthewwellings.com/blog/the-new-vulkan-coordinate-system/
  gl_Position = u_main.proj * u_main.view * u_main.model * vec4(position, 1.0);
  gl_Position.y = -gl_Position.y;
  gl_Position.z = (gl_Position.z + gl_Position.w) / 2.0;

  o_color = color;
}
