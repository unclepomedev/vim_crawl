mod weapons;

use crate::weapons::turret::{base, pitch_barrel, yaw};
use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::sop::SopMerge;

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);

    let base_node = base::build(&mut graph);
    let yaw_node = yaw::build(&mut graph, &base_node);
    let pitch_node = pitch_barrel::build(&mut graph);

    let merge = graph.add(
        SopMerge::new("merge")
            .set_input_at(0, &base_node)
            .set_input_at(1, &yaw_node)
            .set_input_at(2, &pitch_node),
    );

    graph.set_display(&merge);

    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
