use async_trait::async_trait;
use pingora::prelude::*;
use pingora_load_balancing::selection::RoundRobin;
use pingora_load_balancing::LoadBalancer;
use std::sync::Arc;

use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_proxy::{ProxyHttp, Session, http_proxy_service};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0.select(b"", 256).unwrap();

        println!("upstream: {:?}", upstream);

        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }
}

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let upstreams = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();
    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
    lb.add_tcp("0.0.0.0:6188");

    my_server.add_service(lb);
    my_server.run_forever(); // spawn all the runtime threads and block the main thread
}
