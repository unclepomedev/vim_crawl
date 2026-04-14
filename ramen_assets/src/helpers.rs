use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::NodeOutput;
use houdini_ramen::sop::SopRopGltf;

pub struct BuildGraphOutput {
    pub graph: NodeGraph,
    pub last_node: NodeOutput,
    pub display_node: NodeOutput,
}

pub fn add_glb_export_node(graph: &mut NodeGraph, name: &str, input: &NodeOutput, file_name: &str) {
    let export_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("assets")
        .join("models")
        .join(file_name);
    let export_path = export_path.to_string_lossy().into_owned();

    let _rop = graph.add(
        SopRopGltf::new(name)
            .set_input(input)
            .with_file(&export_path),
    );
}
