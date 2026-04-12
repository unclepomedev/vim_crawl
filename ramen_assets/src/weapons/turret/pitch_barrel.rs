use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::{
    SopBoolean, SopBooleanBooleanop, SopNormal, SopNormalMethod, SopPolybevel, SopTube,
    SopTubeOrient, SopTubeType, SopXform,
};

pub fn build(graph: &mut NodeGraph) -> NodeOutput {
    let pitch_joint = graph.add(
        SopTube::new("pitch_joint")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::XAxis)
            .with_rad([0.4, 0.4])
            .with_height(1.3)
            .with_cols(16)
            .with_cap(true),
    );

    let barrel_base = graph.add(
        SopTube::new("barrel_base")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::ZAxis)
            .with_rad([0.15, 0.15])
            .with_height(3.0)
            .with_cols(8)
            .with_cap(true),
    );

    let barrel_offset = graph.add(
        SopXform::new("barrel_offset")
            .set_input(&barrel_base)
            .with_t([0.0, 0.0, 1.5]),
    );

    let bool_pitch_barrel = graph.add(
        SopBoolean::new("bool_pitch_barrel")
            .set_input(&pitch_joint)
            .set_input_at(1, &barrel_offset)
            .with_booleanop(SopBooleanBooleanop::Union),
    );

    let pitch_bevel = graph.add(
        SopPolybevel::new("pitch_bevel")
            .set_input(&bool_pitch_barrel)
            .with_ignoreflatedges(true)
            .with_offset(0.015)
            .with_divisions(2),
    );

    let pitch_normals = graph.add(
        SopNormal::new("pitch_normals")
            .set_input(&pitch_bevel)
            .with_method(SopNormalMethod::ByFaceArea)
            .with_cuspangle(40.0),
    );

    let pitch_mount = graph.add(
        SopXform::new("pitch_mount")
            .set_input(&pitch_normals)
            .with_t([0.0, 2.2, 0.0]),
    );
    NodeOutput::from(&pitch_mount)
}
