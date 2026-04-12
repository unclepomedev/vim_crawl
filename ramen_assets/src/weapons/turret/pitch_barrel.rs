use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::{
    SopBoolean, SopBooleanBooleanop, SopBox, SopColor, SopCopyxform, SopMatchsize,
    SopMatchsizeGoalY, SopMatchsizeJustifyX, SopMatchsizeJustifyY, SopMatchsizeJustifyZ, SopNormal,
    SopNormalMethod, SopTube, SopTubeOrient, SopTubeType, SopXform,
};

pub fn build(graph: &mut NodeGraph, base_node: impl Into<NodeOutput>) -> NodeOutput {
    let pitch_joint = graph.add(
        SopTube::new("pitch_joint")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::XAxis)
            .with_rad([0.4, 0.4])
            .with_height(1.3)
            .with_cols(16)
            .with_cap(true),
    );

    let inner_barrel = graph.add(
        SopTube::new("inner_barrel")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::ZAxis)
            .with_rad([0.1, 0.1])
            .with_height(3.4)
            .with_cols(8)
            .with_cap(true),
    );

    let outer_barrel = graph.add(
        SopTube::new("outer_barrel")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::ZAxis)
            .with_rad([0.18, 0.18])
            .with_height(2.6)
            .with_cols(8)
            .with_cap(true),
    );

    let rail_cutter_box = graph.add(SopBox::new("rail_cutter_box").with_size([0.12, 0.6, 4.0]));
    let rail_cutter_array = graph.add(
        SopCopyxform::new("rail_cutter_array")
            .set_input(&rail_cutter_box)
            .with_ncy(2)
            .with_r([0.0, 0.0, 90.0]),
    );

    let bool_rails = graph.add(
        SopBoolean::new("bool_rails")
            .set_input(&outer_barrel)
            .set_input_at(1, &rail_cutter_array)
            .with_booleanop(SopBooleanBooleanop::Subtract),
    );

    let muzzle_solid = graph.add(
        SopTube::new("muzzle_solid")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::ZAxis)
            .with_rad([0.22, 0.22])
            .with_height(0.5)
            .with_cols(8)
            .with_cap(true),
    );
    let muzzle_pos = graph.add(
        SopXform::new("muzzle_pos")
            .set_input(&muzzle_solid)
            .with_t([0.0, 0.0, 1.45]),
    );

    let barrel_combo = graph.add(
        SopBoolean::new("barrel_combo")
            .set_input(&inner_barrel)
            .set_input_at(1, &bool_rails)
            .with_booleanop(SopBooleanBooleanop::Union),
    );
    let solid_full_barrel = graph.add(
        SopBoolean::new("solid_full_barrel")
            .set_input(&barrel_combo)
            .set_input_at(1, &muzzle_pos)
            .with_booleanop(SopBooleanBooleanop::Union),
    );

    let hole_cutter = graph.add(
        SopTube::new("hole_cutter")
            .with_type(SopTubeType::Polygon)
            .with_orient(SopTubeOrient::ZAxis)
            .with_rad([0.1, 0.06])
            .with_height(1.0)
            .with_cols(8)
            .with_cap(true),
    );
    let hole_pos = graph.add(
        SopXform::new("hole_pos")
            .set_input(&hole_cutter)
            .with_t([0.0, 0.0, 1.5]),
    );
    let hollow_barrel = graph.add(
        SopBoolean::new("hollow_barrel")
            .set_input(&solid_full_barrel)
            .set_input_at(1, &hole_pos)
            .with_booleanop(SopBooleanBooleanop::Subtract),
    );

    let barrel_match = graph.add(
        SopMatchsize::new("barrel_match")
            .set_input(&hollow_barrel)
            .with_justify_x(SopMatchsizeJustifyX::None)
            .with_justify_y(SopMatchsizeJustifyY::None)
            .with_justify_z(SopMatchsizeJustifyZ::Min),
    );

    let barrel_embed = graph.add(
        SopXform::new("barrel_embed")
            .set_input(&barrel_match)
            .with_t([0.0, 0.0, -0.2]),
    );

    let bool_pitch_barrel = graph.add(
        SopBoolean::new("bool_pitch_barrel")
            .set_input(&pitch_joint)
            .set_input_at(1, &barrel_embed)
            .with_booleanop(SopBooleanBooleanop::Union),
    );

    let pitch_normals = graph.add(
        SopNormal::new("pitch_normals")
            .set_input(&bool_pitch_barrel)
            .with_method(SopNormalMethod::ByFaceArea)
            .with_cuspangle(40.0),
    );

    let pitch_match_base = graph.add(
        SopMatchsize::new("pitch_match_base")
            .set_input(&pitch_normals)
            .set_input_at(1, base_node)
            .with_justify_x(SopMatchsizeJustifyX::None)
            .with_justify_y(SopMatchsizeJustifyY::Center)
            .with_goal_y(SopMatchsizeGoalY::Max)
            .with_justify_z(SopMatchsizeJustifyZ::None),
    );

    let pitch_mount = graph.add(
        SopXform::new("pitch_mount")
            .set_input(&pitch_match_base)
            .with_t([0.0, 0.7, 0.0]),
    );

    let pitch_color = graph.add(
        SopColor::new("pitch_color")
            .set_input(&pitch_mount)
            .with_color([0.2, 0.2, 0.2]),
    );

    NodeOutput::from(&pitch_color)
}
