use assert_cmd::Command;
use predicates::prelude::predicate;
use tempfile::TempDir;

fn cmd() -> Command { Command::cargo_bin("rara-cli-template").expect("binary should exist") }

#[test]
fn setup_creates_project_with_all_flags() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let project_dir = tmp.path().join("my-test-cli");

    cmd()
        .args([
            "setup",
            "my-test-cli",
            "--org",
            "testorg",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Project my-test-cli created"));

    // Verify key files exist
    assert!(project_dir.join("Cargo.toml").exists());
    assert!(project_dir.join("src/main.rs").exists());
    assert!(project_dir.join("src/cli/mod.rs").exists());
    assert!(project_dir.join("CLAUDE.md").exists());
    assert!(project_dir.join("justfile").exists());

    // Verify placeholders were replaced
    let cargo_toml = std::fs::read_to_string(project_dir.join("Cargo.toml")).unwrap();
    assert!(cargo_toml.contains("my-test-cli"));
    assert!(!cargo_toml.contains("{{project-name}}"));

    let main_rs = std::fs::read_to_string(project_dir.join("src/main.rs")).unwrap();
    assert!(main_rs.contains("my_test_cli"));
    assert!(!main_rs.contains("{{crate_name}}"));

    // Verify git was initialized
    assert!(project_dir.join(".git").exists());
}

#[test]
fn setup_rejects_invalid_project_name() {
    let tmp = TempDir::new().expect("failed to create temp dir");

    cmd()
        .args([
            "setup",
            "My_Bad_Name",
            "--org",
            "testorg",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("kebab-case"));
}

#[test]
fn setup_rejects_existing_directory() {
    let tmp = TempDir::new().expect("failed to create temp dir");
    std::fs::create_dir(tmp.path().join("already-exists")).unwrap();

    cmd()
        .args([
            "setup",
            "already-exists",
            "--org",
            "testorg",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn setup_replaces_github_org() {
    let tmp = TempDir::new().expect("failed to create temp dir");

    cmd()
        .args([
            "setup",
            "org-test",
            "--org",
            "mycompany",
            "--path",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    let project_dir = tmp.path().join("org-test");
    let package_json = std::fs::read_to_string(project_dir.join("npm/package.json")).unwrap();
    assert!(package_json.contains("mycompany"));
    assert!(!package_json.contains("{{github-org}}"));
}
