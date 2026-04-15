use crate::helpers::BuildGraphOutput;
use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::core::types::{NodeOutput, SpareFloat};
use houdini_ramen::sop::{
    SopAttribwrangle, SopAttribwrangleClass, SopMerge, SopNormal, SopNormalMethod, SopPack,
    SopTexture,
};

pub mod base;
pub mod pitch_barrel;
pub mod yaw;

pub fn build_turret() -> BuildGraphOutput {
    let mut graph = NodeGraph::new("/obj/turret")
        .with_auto_clear()
        .with_auto_create(Geo);

    let base_node = base::build(&mut graph);
    let yaw_node = yaw::build(&mut graph, &base_node);
    let pitch_node = pitch_barrel::build(&mut graph, &base_node);

    let pack_base = graph.add(SopPack::new("pack_base").set_input(&base_node));
    let name_base = graph.add(
        SopAttribwrangle::new("name_base")
            .set_input(&pack_base)
            .with_class(SopAttribwrangleClass::Primitives)
            .with_snippet("s@name = \"base\";"),
    );

    let pack_yaw = graph.add(SopPack::new("pack_yaw").set_input(&yaw_node));
    let name_yaw = graph.add(
        SopAttribwrangle::new("name_yaw")
            .set_input(&pack_yaw)
            .with_class(SopAttribwrangleClass::Primitives)
            .with_snippet("s@name = \"yaw\";"),
    );

    let pack_pitch = graph.add(SopPack::new("pack_pitch").set_input(&pitch_node));
    let name_pitch = graph.add(
        SopAttribwrangle::new("name_pitch")
            .set_input(&pack_pitch)
            .with_class(SopAttribwrangleClass::Primitives)
            .with_snippet("s@name = \"barrel\";"),
    );

    let merge = graph.add(
        SopMerge::new("merge")
            .set_input_at(0, &name_base)
            .set_input_at(1, &name_yaw)
            .set_input_at(2, &name_pitch),
    );

    let calc_normals = graph.add(
        SopNormal::new("calc_normals")
            .set_input(&merge)
            .with_method(SopNormalMethod::ByFaceArea)
            .with_cuspangle(40.0),
    );

    let data_flow_fx = graph.add(
        SopAttribwrangle::new("data_flow_fx")
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

    let dummy_uv = graph.add(SopTexture::new("dummy_uv").set_input(&data_flow_fx));

    BuildGraphOutput {
        graph,
        last_node: NodeOutput::from(&dummy_uv),
        display_node: NodeOutput::from(&data_flow_fx),
    }
}
