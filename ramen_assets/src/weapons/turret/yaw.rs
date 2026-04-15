use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::{
    SopBoolean, SopBooleanBooleanop, SopBox, SopColor, SopMatchsize, SopMatchsizeGoalY,
    SopMatchsizeJustifyY, SopTube, SopTubeType, SopXform,
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

    // Create a cradle for the pitch joint and mount it flush to the base top.
    let yaw_mount = graph.add(
        SopMatchsize::new("yaw_mount")
            .set_input(&bool_yaw)
            .set_input_at(1, base_node)
            .with_justify_y(SopMatchsizeJustifyY::Min)
            .with_goal_y(SopMatchsizeGoalY::Max),
    );

    let yaw_color = graph.add(
        SopColor::new("yaw_color")
            .set_input(&yaw_mount)
            .with_color([0.0, 1.0, 0.0]),
    );

    NodeOutput::from(&yaw_color)
}
