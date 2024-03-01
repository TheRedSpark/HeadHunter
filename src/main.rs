use clap::Parser;
use env_logger::{Builder, WriteStyle};
use log::{info, LevelFilter, warn};
use warp::Filter;
pub const PORT: u16 = 8443;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    debug: i8,
}
#[tokio::main]
async fn main() {
    let mut builder = Builder::new();
    builder
        .filter(None, LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();
    let mut is_tls: bool = true;
    let args: Args = Args::parse();

    println!("{}", args.debug);
    if args.debug == 1 {
        warn!("Debug ist enabled");
        warn!("Es wird kein TLS verwendet");
        is_tls = false
    } else { info!("Normal Startup with TLS") }
    let headers_filter = warp::header::headers_cloned()
        .map(|headers: warp::http::HeaderMap| {
            for (key, value) in headers.iter() {
                println!("{:?}: {:?}", key, value);
            }
        });

    let routes = warp::any()
        .and(headers_filter)
        .map(|_| warp::reply::html("Header wurden in der Konsole ausgegeben."));
    println!("Headers Hunter gestartet");
    if is_tls {
        warp::serve(routes)
            .tls()
            .cert_path("fullchain.pem")
            .key_path("privkey.pem")
            .run(([0, 0, 0, 0], PORT))
            .await;
    } else {
        warp::serve(routes)
            .run(([0, 0, 0, 0], PORT))
            .await;
    }
}
