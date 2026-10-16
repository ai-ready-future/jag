//! End-to-end test of issue 160: a call to a gateway on this machine with the
//! scheme `http` never goes to a proxy of the environment, because the gateway
//! key would then leave this machine with no TLS. The tests use a fake server,
//! a fake proxy that only counts the connections, and the made-up key of
//! `common::KEY`.

mod common;

use std::net::TcpListener;
use std::time::Duration;

use common::fake_server::FakeServer;
use common::{KEY, jag, write};

const PROXY_VARIABLES: [&str; 6] = [
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "ALL_PROXY",
    "http_proxy",
    "https_proxy",
    "all_proxy",
];

/// A listener that stands for a proxy. The test only asks if a client
/// connected to it.
struct FakeProxy {
    listener: TcpListener,
}

impl FakeProxy {
    fn start() -> FakeProxy {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind the fake proxy");
        listener.set_nonblocking(true).expect("non-blocking");
        FakeProxy { listener }
    }

    fn address(&self) -> String {
        format!(
            "http://127.0.0.1:{}",
            self.listener.local_addr().unwrap().port()
        )
    }

    /// True when a client connected.
    fn was_used(&self) -> bool {
        self.listener.accept().is_ok()
    }
}

// Issue 160: the request goes to the gateway on this machine, not to the proxy.
#[test]
fn an_http_gateway_on_this_machine_is_called_with_no_proxy() {
    let dir = tempfile::tempdir().expect("temporary folder");
    write(dir.path(), "a.txt", b"needle\n");
    for name in PROXY_VARIABLES {
        let server = FakeServer::constant(0.8);
        let proxy = FakeProxy::start();
        let home = tempfile::tempdir().expect("temporary folder");
        let output = jag(home.path(), &server.root())
            .env(name, proxy.address())
            .args(["--ask", "which line?", "needle"])
            .arg(dir.path())
            .timeout(Duration::from_secs(30))
            .output()
            .expect("run jag");
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let used = proxy.was_used();
        let calls = server.requests().len();
        assert_eq!(
            (output.status.code(), calls, used),
            (Some(0), 1, false),
            "{name}: (exit code, calls to the gateway, proxy used) {stderr}"
        );
        assert!(!stderr.contains(KEY) && !String::from_utf8_lossy(&output.stdout).contains(KEY));
    }
}
