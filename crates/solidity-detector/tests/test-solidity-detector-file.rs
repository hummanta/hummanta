use std::process::Command;

#[test]
fn test_solidity_detector_file() {
    let path = "../../samples/solidity/simple-storage";
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "solidity-detector",
            "--bin",
            "solidity-detector-file",
            "--",
            "--path",
            path,
        ])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).expect("stdout is not valid UTF-8");
    assert_eq!(stdout, "{\"pass\":true,\"language\":\"Solidity\"}\n");
}
