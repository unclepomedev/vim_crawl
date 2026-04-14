mod helpers;
mod weapons;

use crate::helpers::export_glb;
use crate::weapons::turret;
use houdini_ramen::core::live_link::send_to_houdini;

fn main() {
    let mut result = turret::build_turret();
    export_glb(
        &mut result.graph,
        "export_turret",
        &result.last_node,
        "turret.glb",
    );
    result.graph.set_display(&result.display_node);

    let python_script = result.graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
