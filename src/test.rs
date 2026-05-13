use zed_extension_api as zed;

fn main() {
    let _ = zed::extension::dap::DebugAdapterBinary {
        command: None,
    };
}
