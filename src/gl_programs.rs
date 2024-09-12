use bevy_color::Oklcha;
use eframe::glow::{self, HasContext};
use strum::EnumIter;

#[derive(Default, Clone, Copy, EnumIter, Hash, PartialEq, Eq)]
pub enum ProgramKind {
    #[default]
    Picker,
    Picker2,
    Hue,
    Lightness,
    Chroma,
    Alpha,
    FinalPrevious,
    Final,
}

pub struct GlowProgram {
    kind: ProgramKind,
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl GlowProgram {
    pub fn new(gl: &glow::Context, kind: ProgramKind) -> Self {
        unsafe {
            let program = gl.create_program().unwrap();
            let vert_shader_source = include_str!("./shaders/quad_vert.glsl");

            let frag_shader_source = match kind {
                ProgramKind::Picker => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/picker_frag.glsl")
                ),
                ProgramKind::Picker2 => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/picker2_frag.glsl")
                ),
                ProgramKind::Hue => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/hue_frag.glsl")
                ),
                ProgramKind::Lightness => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/lightness_frag.glsl")
                ),
                ProgramKind::Chroma => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/chroma_frag.glsl")
                ),
                ProgramKind::Alpha => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/alpha_frag.glsl")
                ),
                ProgramKind::Final | ProgramKind::FinalPrevious => concat!(
                    include_str!("shaders/functions.glsl"),
                    include_str!("shaders/final_frag.glsl")
                ),
            };

            let shader_sources = [
                (glow::VERTEX_SHADER, vert_shader_source),
                (glow::FRAGMENT_SHADER, frag_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, shader_source);
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                kind,
                program,
                vertex_array,
            }
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    pub fn paint(
        &self,
        gl: &glow::Context,
        color: Oklcha,
        fallback_color: [f32; 4],
        previous_fallback_color: [f32; 4],
        width: f32,
    ) {
        unsafe {
            let set_uni_f32 = |name: &str, value: f32| {
                gl.uniform_1_f32(gl.get_uniform_location(self.program, name).as_ref(), value);
            };
            gl.use_program(Some(self.program));
            set_uni_f32("width", width);
            match self.kind {
                ProgramKind::Picker => {
                    set_uni_f32("hue", color.hue);
                }
                ProgramKind::Picker2 => {
                    set_uni_f32("lightness", color.lightness);
                }
                ProgramKind::Hue => {}
                ProgramKind::Lightness => {
                    set_uni_f32("hue", color.hue);
                    set_uni_f32("chroma", color.chroma);
                }
                ProgramKind::Chroma => {
                    set_uni_f32("hue", color.hue);
                    set_uni_f32("lightness", color.lightness);
                }
                ProgramKind::Alpha => {
                    gl.uniform_3_f32_slice(
                        gl.get_uniform_location(self.program, "color").as_ref(),
                        &fallback_color[0..3][..],
                    );
                }
                ProgramKind::Final => {
                    gl.uniform_4_f32_slice(
                        gl.get_uniform_location(self.program, "color").as_ref(),
                        &fallback_color[..],
                    );
                }
                ProgramKind::FinalPrevious => {
                    gl.uniform_4_f32_slice(
                        gl.get_uniform_location(self.program, "color").as_ref(),
                        &previous_fallback_color[..],
                    );
                }
            }
            gl.bind_vertex_array(Some(self.vertex_array));

            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}
