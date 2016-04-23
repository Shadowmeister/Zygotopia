#version 140

in vec3 position;
in vec3 normal;
in int mat_id;

out vec3 v_normal;
out vec3 v_position;
flat out int v_mat_id;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
  mat4 modelview = view * model;
  v_mat_id = mat_id;
  v_normal = transpose(inverse(mat3(modelview))) * normal;
  gl_Position = perspective * modelview * vec4(position, 1.0);
  v_position = gl_Position.xyz / gl_Position.w;
}
