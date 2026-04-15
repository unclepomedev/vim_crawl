use crate::helpers::BuildGraphOutput;
use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::core::types::{NodeOutput, SpareFloat};
use houdini_ramen::sop::{
    SopAttribwrangle, SopAttribwrangleClass, SopMerge, SopNormal, SopNormalMethod, SopPack,
    SopTexture, SopXform,
};

pub mod base;
pub mod pitch_barrel;
pub mod yaw;

fn process_and_pack(
    graph: &mut NodeGraph,
    input: &NodeOutput,
    part_name: &str,
    export_name: &str,
) -> NodeOutput {
    let calc_normals = graph.add(
        SopNormal::new(&format!("normals_{}", part_name))
            .set_input(input)
            .with_method(SopNormalMethod::ByFaceArea)
            .with_cuspangle(40.0),
    );

    let data_flow_fx = graph.add(
        SopAttribwrangle::new(&format!("fx_{}", part_name))
            .set_input(&calc_normals)
            .with_snippet(include_str!("data_flow_fx.vfl"))
            .add_spare(
                SpareFloat::new("noise_scale", "Noise Scale")
                    .with_default(5.0)
                    .with_range(0.1, 20.0),
            )
            .add_spare(
                SpareFloat::new("noise_intensity", "Noise Intensity")
                    .with_default(1.0)
                    .with_range(0.0, 2.0),
            ),
    );

    let dummy_uv =
        graph.add(SopTexture::new(&format!("uv_{}", part_name)).set_input(&data_flow_fx));

    let pack = graph.add(SopPack::new(&format!("pack_{}", part_name)).set_input(&dummy_uv));

    let name_attr = graph.add(
        SopAttribwrangle::new(&format!("name_{}", part_name))
            .set_input(&pack)
            .with_class(SopAttribwrangleClass::Primitives)
            .with_snippet(&format!("s@name = \"{}\";", export_name)),
    );

    NodeOutput::from(&name_attr)
}

pub fn build_turret() -> BuildGraphOutput {
    let mut graph = NodeGraph::new("/obj/turret")
        .with_auto_clear()
        .with_auto_create(Geo);

    let base_node = base::build(&mut graph);
    let yaw_node = yaw::build(&mut graph, &base_node);
    let pitch_node = pitch_barrel::build(&mut graph, &base_node);

    let processed_base = process_and_pack(&mut graph, &base_node, "base", "base");
    let processed_yaw = process_and_pack(&mut graph, &yaw_node, "yaw", "yaw");
    let processed_barrel = process_and_pack(&mut graph, &pitch_node, "pitch", "pitch");

    let merge = graph.add(
        SopMerge::new("merge")
            .set_input_at(0, &processed_base)
            .set_input_at(1, &processed_yaw)
            .set_input_at(2, &processed_barrel),
    );

    let normalize_all = graph.add(
        SopXform::new("normalize_all")
            .set_input(&merge)
            .with_scale(0.25),
    );

    BuildGraphOutput {
        graph,
        last_node: NodeOutput::from(&normalize_all),
        display_node: NodeOutput::from(&normalize_all),
    }
}
