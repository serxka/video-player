use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use structopt::StructOpt;

#[derive(StructOpt, Clone)]
struct Options {
    #[structopt(short = "p", long = "host", default_value = "localhost:8080")]
    host: String,
    #[structopt(short = "v", long = "videos", default_value = "./")]
    path: String,
}

async fn player(video_path: web::Data<String>) -> HttpResponse {
    let paths = std::fs::read_dir(video_path.get_ref())
        .unwrap()
        .filter_map(|v| v.ok())
        .filter(|v| {
            v.path()
                .extension()
                .unwrap_or(std::ffi::OsStr::new(" "))
                .eq_ignore_ascii_case("mp4")
        })
        .map(|v| v.path().file_name().unwrap().to_os_string());
    let mut files = String::new();
    for path in paths {
        files.push_str("\"/static/");
        files.push_str(&path.into_string().unwrap());
        files.push_str("\", ");
    }
    let playlist = std::fs::read_to_string(format!("{}/playlist.json", video_path.get_ref()))
        .unwrap_or_else(|_| "null".to_owned());
    files.truncate(files.len() - 2);
    let html = include_str!("index.html")
        .replace("{{videos}}", &files)
        .replace("{{playlist}}", &playlist);
    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts = Options::from_args();

    let path: std::path::PathBuf = opts.path.clone().into();
    if !path.exists() {
        eprintln!("directory '{}' doesn't exist", path.display());
        std::process::exit(1);
    }

    let listener = std::net::TcpListener::bind(&opts.host)?;
    let addr = listener.local_addr()?;
    println!(
        "Listening on address: '{:?}' serving directory: '{}'",
        addr, opts.path
    );

    HttpServer::new(move || {
        use actix_web::web::{get, resource};
        App::new()
            .service(resource("/").route(get().to(player)))
            .service(Files::new("/static", &opts.path).redirect_to_slash_directory())
            .data(opts.path.clone())
    })
    .listen(listener)?
    .run()
    .await
}
