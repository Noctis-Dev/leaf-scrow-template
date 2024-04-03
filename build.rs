use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use io::ScrowMetadata;

fn main() {
    WasmBuilder::with_meta(ScrowMetadata::repr())
        .exclude_features(vec!["binary-vendor"]).build();
}