pub fn _get_operating_system() -> String {
    match std::env::consts::OS {
        "linux" => "linux".to_string(),
        "macos" => "macos".to_string(),
        "windows" => "windows".to_string(),
        _ => "unknown".to_string(),
    }
}
