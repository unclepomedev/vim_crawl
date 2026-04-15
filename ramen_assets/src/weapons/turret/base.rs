use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::{
    SopBoolean, SopBooleanBooleanop, SopBox, SopColor, SopCopyxform, SopMatchsize,
    SopMatchsizeJustifyY, SopTube, SopTubeType,
};

pub fn build(graph: &mut NodeGraph) -> NodeOutput {
    // Radii (1.5, 2.0) define the base scale, which will be normalized to 1.0m logic.
    let core_tube = graph.add(
        SopTube::new("core_tube")
            .with_type(SopTubeType::Polygon)
            .with_rad([1.5, 1.5])
            .with_height(1.5)
            .with_cols(16)
            .with_cap(true),
    );

    let core_ground = graph.add(
        SopMatchsize::new("core_ground")
            .set_input(&core_tube)
            .with_justify_y(SopMatchsizeJustifyY::Min),
    );

    let armor_tube = graph.add(
        SopTube::new("armor_tube")
            .with_type(SopTubeType::Polygon)
            .with_rad([2.0, 2.0])
            .with_height(1.3)
            .with_cols(8)
            .with_cap(true),
    );

    let armor_ground = graph.add(
        SopMatchsize::new("armor_ground")
            .set_input(&armor_tube)
            .with_justify_y(SopMatchsizeJustifyY::Min),
    );

    let armor_color = graph.add(
        SopColor::new("armor_color")
            .set_input(&armor_ground)
            .with_color([0.0, 1.0, 0.0]),
    );

    let slice_box = graph.add(SopBox::new("slice_box").with_size([0.15, 2.1, 5.0]));

    let slice_array = graph.add(
        SopCopyxform::new("slice_array")
            .set_input(&slice_box)
            .with_ncy(4)
            .with_r([0.0, 45.0, 0.0]),
    );

    // Subtract slots and fins to create mechanical detail via "BInsideA" group coloring.
    let bool_slice = graph.add(
        SopBoolean::new("bool_slice")
            .set_input(&armor_color)
            .set_input_at(1, &slice_array)
            .with_booleanop(SopBooleanBooleanop::Subtract)
            .with_binsidea("BInsideA"),
    );

    let fin_box = graph.add(
        SopBox::new("fin_box")
            .with_size([5.0, 0.05, 5.0])
            .with_t([0.0, 0.2, 0.0]),
    );

    let fin_array = graph.add(
        SopCopyxform::new("fin_array")
            .set_input(&fin_box)
            .with_ncy(8)
            .with_t([0.0, 0.12, 0.0]),
    );

    let group_inside_color = graph.add(
        SopColor::new("group_inside_color")
            .set_input(&bool_slice)
            .with_group("BInsideA")
            .with_color([0.0, 0.0, 0.0]),
    );

    let bool_fins = graph.add(
        SopBoolean::new("bool_fins")
            .set_input(&group_inside_color)
            .set_input_at(1, &fin_array)
            .with_booleanop(SopBooleanBooleanop::Subtract)
            .with_binsidea("BInsideA"),
    );

    let core_color = graph.add(
        SopColor::new("core_color")
            .set_input(&core_ground)
            .with_color([0.0, 0.0, 0.0]),
    );

    let black = graph.add(
        SopColor::new("black")
            .set_input(&bool_fins)
            .with_group("BInsideA")
            .with_color([0.0, 0.0, 0.0]),
    );

    let base_union = graph.add(
        SopBoolean::new("base_union")
            .set_input(&core_color)
            .set_input_at(1, &black)
            .with_booleanop(SopBooleanBooleanop::Union),
    );

    NodeOutput::from(&base_union)
}
