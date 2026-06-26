fn main() {
    // Build WASM
    if let Some((_, wasm_path)) = sails_rs::build_wasm() {
        // Generate IDL and embed it into WASM
        sails_rs::ClientBuilder::<::agent_calculator_app::Program>::from_wasm_path(
            wasm_path.with_extension(""),
        )
        .build_idl();
    }
}
