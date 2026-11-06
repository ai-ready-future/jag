//! End-to-end tests of STORY-86 (FEATURE-21): a call to the gateway base URL
//! with no key (CRITERION-155, CRITERION-156, and CRITERION-157). The tests
//! run the built `jag` as a user does, with a temporary `HOME` and the fake
//! server only.
//!
//! `tests/gateway.rs`, `tests/check.rs`, and `tests/score.rs` already cover
//! the main cases: a search with no key, with the TypeSafe key set, a 401 or a
//! 403 with `JAG_BASE_URL` and with `--base-url`, the JSON error document, the
//! check with 6 lines, and its 401, 402, 403, and timeout cases. These tests
//! add the cases that no other file has.

mod common;

use std::net::TcpListener;
use std::process::Output;

use common::fake_server::FakeServer;
use common::{jag_bare, write, write_config};
use tempfile::TempDir;

fn folder() -> TempDir {
    tempfile::tempdir().expect("temporary folder")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn needle_folder() -> TempDir {
    let dir = folder();
    write(dir.path(), "a.txt", b"needle\n");
    dir
}

/// Run `jag --ask q needle <dir>` against the server with no gateway key,
/// with these variables and this text in the config file.
fn ask(server: &FakeServer, config: Option<&str>, variables: &[(&str, &str)]) -> Output {
    let home = folder();
    if let Some(text) = config {
        write_config(home.path(), text);
    }
    let dir = needle_folder();
    let mut command = jag_bare(home.path());
    command.env("JAG_BASE_URL", server.base_url());
    for (name, value) in variables {
        command.env(name, value);
    }
    command
        .args(["--ask", "q", "needle"])
        .arg(dir.path())
        .output()
        .expect("run jag")
}

fn assert_one_call_with_no_authorization(server: &FakeServer, output: &Output) {
    assert_eq!(output.status.code(), Some(0), "{}", stderr(output));
    let requests = server.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].header("authorization"), None);
}

// Row 2: a blank variable counts as not set, so the call has no key.
#[test]
fn a_blank_gateway_key_variable_gives_a_call_with_no_authorization() {
    for blank in ["", "  "] {
        let server = FakeServer::constant(0.8);
        let output = ask(&server, None, &[("JAG_GATEWAY_API_KEY", blank)]);
        assert_one_call_with_no_authorization(&server, &output);
    }
}

// Row 2: a blank value in the file counts as not set, so the call has no key.
#[test]
fn a_blank_gateway_key_in_the_file_gives_a_call_with_no_authorization() {
    for text in [
        "config:\n  gateway-api-key: \"\"\n",
        "config:\n  gateway-api-key:\n",
        "config:\n  gateway-api-key: \"  \"\n",
    ] {
        let server = FakeServer::constant(0.8);
        let output = ask(&server, Some(text), &[]);
        assert_one_call_with_no_authorization(&server, &output);
    }
}

/// An address on 127.0.0.1 where no server listens.
fn closed_address() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().expect("address").port();
    drop(listener);
    format!("http://127.0.0.1:{port}")
}

// Row 19: a port with no server fails the reachable line (exit 6), and the
// scoring line is `not checked`.
#[test]
fn the_check_on_a_port_with_no_server_fails_the_reachable_line_with_exit_6() {
    let home = folder();
    let output = jag_bare(home.path())
        .env("JAG_BASE_URL", closed_address())
        .env("JAG_MODEL", "jev-1.13.0")
        .env("JAG_TIMEOUT", "1")
        .env("JAG_RETRY_WAIT", "1")
        .arg("--check")
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(6), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "not needed  a key is present\n\
         not needed  the service accepts the key\n\
         not available the account has credit\n\
         fail        the service is reachable\n\
         not checked the server answers a scoring call (the call to the service failed)\n\
         pass        other users cannot read the key file (no key file)\n"
    );
}

// Row 12 and row 3: the direct route with no key keeps its 5 lines, exit 3,
// and makes no call. A dead proxy makes sure that no call can leave the
// machine, if the route sent one by mistake.
#[test]
fn the_check_on_the_direct_route_with_no_key_keeps_five_lines_and_exit_3() {
    let home = folder();
    let output = jag_bare(home.path())
        .env("JAG_MODEL", "jev-1.13.0")
        .env("JAG_TIMEOUT", "1")
        .env("JAG_RETRY_WAIT", "1")
        .env("HTTPS_PROXY", "http://127.0.0.1:9")
        .env("HTTP_PROXY", "http://127.0.0.1:9")
        .arg("--check")
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(3), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "fail        a key is present\n\
         not checked the service accepts the key (jag has no key to send)\n\
         not checked the account has credit (jag has no key to send)\n\
         not checked the service is reachable (jag has no key to send)\n\
         pass        other users cannot read the key file (no key file)\n"
    );
    assert_eq!(
        stderr(&output),
        "jag: no TypeSafe API key found.\n\
         Get a key at https://console.typesafe.ai/keys, then run `jag --setup`,\n\
         or set JAG_TYPESAFE_API_KEY.\n"
    );
}
