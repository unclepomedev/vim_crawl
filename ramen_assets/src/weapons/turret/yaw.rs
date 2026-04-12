use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::{
    SopBoolean, SopBooleanBooleanop, SopBox, SopMatchsize, SopMatchsizeGoalY, SopMatchsizeJustifyY,
    SopNormal, SopNormalMethod, SopPolybevel, SopTube, SopTubeType, SopXform,
};

pub fn build(graph: &mut NodeGraph, base_node: impl Into<NodeOutput>) -> NodeOutput {
    let yaw_tube = graph.add(
        SopTube::new("yaw_tube")
            .with_type(SopTubeType::Polygon)
            .with_rad([1.3, 1.3])
            .with_height(1.2)
            .with_cols(8)
            .with_cap(true),
    );

    let pitch_cutter_box = graph.add(SopBox::new("pitch_cutter_box").with_size([1.2, 1.5, 3.0]));

    let pitch_cutter_pos = graph.add(
        SopXform::new("pitch_cutter_pos")
            .set_input(&pitch_cutter_box)
            .with_t([0.0, 0.6, 0.0]),
    );

    let bool_yaw = graph.add(
        SopBoolean::new("bool_yaw")
            .set_input(&yaw_tube)
            .set_input_at(1, &pitch_cutter_pos)
            .with_booleanop(SopBooleanBooleanop::Subtract),
    );

    let yaw_bevel = graph.add(
        SopPolybevel::new("yaw_bevel")
            .set_input(&bool_yaw)
            .with_ignoreflatedges(true)
            .with_offset(0.02)
            .with_divisions(2),
    );

    let yaw_mount = graph.add(
        SopMatchsize::new("yaw_mount")
            .set_input(&yaw_bevel)
            .set_input_at(1, base_node)
            .with_justify_y(SopMatchsizeJustifyY::Min)
            .with_goal_y(SopMatchsizeGoalY::Max),
    );

    let yaw_normals = graph.add(
        SopNormal::new("yaw_normals")
            .set_input(&yaw_mount)
            .with_method(SopNormalMethod::ByFaceArea)
            .with_cuspangle(40.0),
    );
    NodeOutput::from(&yaw_normals)
}
