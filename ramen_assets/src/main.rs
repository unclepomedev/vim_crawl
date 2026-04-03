use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::generated::sop::SopBox;
use houdini_ramen::generated::sop::{SopAttribwrangle, SopAttribwrangleClass};
use houdini_ramen::helpers::loops::add_foreach_loop;

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);

    let box_node = graph.add(SopBox::new("base_box").with_size([2.0, 2.0, 2.0]));

    let loop_end = add_foreach_loop(&mut graph, "process_points", &box_node, |g, begin| {
        g.add(
            SopAttribwrangle::new("inner_process")
                .set_input(begin)
                .with_class(SopAttribwrangleClass::Primitives)
                .with_snippet(include_str!("vex/001_1_yp1.vfl")),
        )
    });

    let _final_wrangle = graph.add(
        SopAttribwrangle::new("post_process")
            .set_input(&loop_end)
            .with_class(SopAttribwrangleClass::Primitives)
            .with_snippet(include_str!("vex/001_2_set.vfl")),
    );

    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
