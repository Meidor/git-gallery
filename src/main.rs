#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate clap;

mod config;
pub use config::Config;
pub use config::GalleryConfig;
use std::path::{Path, PathBuf};

mod gallery;
pub use gallery::{generate_thumbnails, AlbumContext};

use clap::App;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn get_album_template(path: PathBuf, title: String) -> Option<Template> {
    let gallery_config = get_gallery_config()?;
    if path.exists() && path.is_dir() {
        Some(Template::render(
            "album",
            AlbumContext::new(
                &title,
                &gallery_config.author?,
                &gallery_config.description?,
                &path,
            ),
        ))
    } else {
        None
    }
}

#[get("/")]
fn index() -> Option<Template> {
    get_album_template(PathBuf::from("album/"), get_gallery_config()?.title?)
}

#[get("/album/<path..>")]
fn album(path: PathBuf) -> Option<Template> {
    let album_dir = Path::new("album").join(path);
    get_album_template(
        album_dir.clone(),
        album_dir.file_name()?.to_str()?.to_string(),
    )
}

fn serve() {
    generate_thumbnails();
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, album])
        .mount("/photo", StaticFiles::from("album/"))
        .mount("/assets", StaticFiles::from("assets/"))
        .launch();
}

fn init() {
    println!("INIT");
}

fn build() {
    println!("Building the world");
}

fn get_gallery_config() -> Option<GalleryConfig> {
    config::get_config("gg.toml").gallery
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand() {
        ("serve", Some(_)) => serve(),
        ("init", Some(_)) => init(),
        _ => build(),
    }
}
