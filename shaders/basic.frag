#version 140

in vec3 v_position;
in vec3 v_normal;
flat in int v_mat_id;
out vec4 color;

uniform vec3 u_light;
uniform vec3 diffuse0;
uniform vec3 ambient0;
uniform vec3 specular0;
uniform vec3 diffuse1;
uniform vec3 ambient1;
uniform vec3 specular1;

void main() {
  float diffuse_power = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
  vec3 camera_dir = normalize(-v_position);
  vec3 half_direction = normalize(normalize(u_light) + camera_dir);
  float specular_power = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

  if(v_mat_id == 0) {
    color = vec4(ambient0 + diffuse_power * diffuse0 + specular_power * specular0, 1.0);
  } else if(v_mat_id == 1) {
    color = vec4(ambient1 + diffuse_power * diffuse1 + specular_power * specular1, 1.0);
  }
}
