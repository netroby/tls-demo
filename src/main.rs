extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

fn main() {
    let mut core = ::tokio_core::reactor::Core::new().unwrap();

    let client = ::hyper::Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let res = core.run(client.get("https://hyper.rs".parse().unwrap())).unwrap();
    assert_eq!(res.status(), ::hyper::Ok);
    println!("Hello world!");
    println!("echo {}", 1+2);
}