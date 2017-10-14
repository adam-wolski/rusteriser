use gl;
use utils;
use cgmath::*;

pub fn simple_vertex(inputs: gl::VSInput) -> gl::VSOutput {
    let mut output: gl::VSOutput = gl::VSOutput::default();
    output.position = inputs.projection * inputs.view * inputs.position;
    output.texcoord = inputs.texcoord;
    output.normal = inputs.normal;
    output
}

pub fn simple_pixel(inputs: gl::PSInput) -> Vector4<f32> {
    let normal = inputs.normal;
    let light_dir = inputs.light_pos;
    let n = normal.normalize();
    let l = light_dir.normalize();
    let ndotl = utils::saturate(n.dot(l));
    vec4(ndotl, ndotl, ndotl, 1.0)
}

pub fn diffuse_pixel(inputs: gl::PSInput) -> Vector4<f32> {
    let texcoord = inputs.texcoord;
    utils::sample(&inputs.textures[0], texcoord)
}

pub fn spec_pixel(inputs: gl::PSInput) -> Vector4<f32> {
    let texcoord = inputs.texcoord;
    let normal = inputs.normal;
    let light_dir = inputs.light_pos;
    let cam_dir = inputs.cam_dir;

    let diffuse_tex = utils::sample(&inputs.textures[0], texcoord);
    let normals_tex = utils::sample(&inputs.textures[1], texcoord).truncate();
    let specular_tex = utils::sample(&inputs.textures[2], texcoord).truncate();

    let nrm: Vector3<f32> = Vector3::new(
        normal.x * normals_tex.x,
        normal.y * normals_tex.y,
        normal.z * normals_tex.z,
    );

    let n = nrm.normalize();
    let l = light_dir.normalize();
    let r = utils::reflect(-l, n);
    let e = cam_dir.normalize();
    let ndotl = utils::saturate(n.dot(l));
    let edotr = utils::saturate(e.dot(r));

    let mut spec = specular_tex * edotr;
    spec.x = spec.x.powf(5.0);
    spec.y = spec.y.powf(5.0);
    spec.z = spec.z.powf(5.0);

    let mut ambient = diffuse_tex * 0.1;
    ambient.z *= 1.5;

    utils::saturate_v4(ambient + (diffuse_tex * ndotl) + spec.extend(0.0))
}
