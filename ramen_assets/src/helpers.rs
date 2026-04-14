use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::SopRopGltf;

pub struct BuildGraphOutput {
    pub graph: NodeGraph,
    pub last_node: NodeOutput,
    pub display_node: NodeOutput,
}

pub fn export_glb(graph: &mut NodeGraph, name: &str, input: &NodeOutput, file_name: &str) {
    let export_path = std::env::current_dir()
        .unwrap()
        .join("..")
        .join("assets")
        .join("models")
        .join(file_name);

    graph.add(
        SopRopGltf::new(name)
            .set_input(input)
            .with_file(export_path.to_str().unwrap()),
    );
}
