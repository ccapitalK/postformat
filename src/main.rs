use log::info;
use warp::Filter;

mod format;

#[tokio::main]
async fn main() {
    simple_logging::log_to_file("server.log", log::LevelFilter::Info).expect("Failed to open log file");
    let cpp = warp::put()
        .and(warp::path!("cpp").or(warp::path!("format" / "clang")).unify())
        .and(warp::body::content_length_limit(1024 * 1024))
        .and(warp::body::aggregate())
        .and_then(format::clang_format);

    let python = warp::put()
        .and(warp::path!("py").or(warp::path!("format" / "autopep8")).unify())
        .and(warp::body::content_length_limit(1024 * 1024))
        .and(warp::body::aggregate())
        .and_then(format::autopep8_format);

    let js = warp::put()
        .and(warp::path!("js").or(warp::path!("format" / "js")).unify())
        .and(warp::body::content_length_limit(1024 * 1024))
        .and(warp::body::aggregate())
        .and_then(format::js_format);

    let rust = warp::put()
        .and(warp::path!("rust").or(warp::path!("format" / "rust")).unify())
        .and(warp::body::content_length_limit(1024 * 1024))
        .and(warp::body::aggregate())
        .and_then(format::rust_format);

    info!("Started up");
    warp::serve(cpp.or(python).or(js).or(rust))
        .run(([127, 0, 0, 1], 8080))
        .await;
}
