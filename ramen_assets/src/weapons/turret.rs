use crate::helpers::BuildGraphOutput;
use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::{SopMerge, SopTexture};

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

    let merge = graph.add(
        SopMerge::new("merge")
            .set_input_at(0, &base_node)
            .set_input_at(1, &yaw_node)
            .set_input_at(2, &pitch_node),
    );

    let dummy_uv = graph.add(SopTexture::new("dummy_uv").set_input(&merge));

    BuildGraphOutput {
        graph,
        last_node: NodeOutput::from(&dummy_uv),
        display_node: NodeOutput::from(&merge),
    }
}
