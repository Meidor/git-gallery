use glob::glob;
use image::FilterType;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

static THUMBNAIL_FILE: &str = "thumbnail.jpg";

pub fn generate_thumbnails() {
    for album in WalkDir::new("album")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|x| x.path().is_dir())
    {
        let photo = get_images(&album.path().to_path_buf()).next().unwrap();
        let filename = album.path().join(THUMBNAIL_FILE);
        let height = 400;
        image::open(photo)
            .unwrap()
            .resize(::std::u32::MAX, height, FilterType::Lanczos3)
            .save(filename)
            .ok();
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Photo {
    pub path: String,
}

impl Photo {
    pub fn new(path: String) -> Photo {
        Photo { path: path }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Album {
    pub title: String,
    pub path: String,
    pub thumbnail: String,
}

impl Album {
    pub fn new(path: String, title: String, thumbnail: String) -> Album {
        Album {
            path: path,
            title: title,
            thumbnail: thumbnail,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AlbumContext<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    pub albums: Vec<Album>,
    pub photos: Vec<Photo>,
}

impl<'a> AlbumContext<'a> {
    pub fn new(
        title: &'a str,
        author: &'a str,
        description: &'a str,
        directory: &'a PathBuf,
    ) -> AlbumContext<'a> {
        AlbumContext {
            title: title,
            author: author,
            description: description,
            albums: AlbumContext::get_albums(directory),
            photos: AlbumContext::get_photos(directory),
        }
    }

    fn get_albums(dir: &'a PathBuf) -> Vec<Album> {
        fs::read_dir(dir)
            .unwrap()
            .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_dir())
            .map(|x| {
                let path = x.unwrap().path();
                let path_string = get_path_string(path.clone());
                let thumbnail_string = "/photo".to_string() + &path_string[6..] + "/thumbnail.jpg";
                Album::new(
                    path_string,
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    thumbnail_string,
                )
            })
            .collect()
    }

    fn get_photos(dir: &'a PathBuf) -> Vec<Photo> {
        get_images(dir)
            .filter(|p| p.file_name().unwrap() != THUMBNAIL_FILE)
            .map(|p| {
                let mut path = String::from("/photo/");
                path.push_str(&get_path_string(p)[7..]);
                Photo::new(path)
            })
            .collect()
    }
}

fn get_path_string(path: PathBuf) -> String {
    path.components().fold(String::new(), |mut c, n| {
        c.push('/');
        c.push_str(n.as_os_str().to_str().unwrap());
        c
    })
}

fn get_image_glob(dir: &PathBuf, extension: &str) -> String {
    let mut path_string = get_path_string(dir.to_path_buf())[1..].to_string();
    let extension_glob = &format!("*.{}", extension);
    path_string.push('/');
    path_string.push_str(extension_glob);
    path_string
}
type ImageIter = Box<dyn Iterator<Item = PathBuf>>;

fn get_images(dir: &PathBuf) -> ImageIter {
    let jpg = glob(&get_image_glob(dir, "jpg"))
        .unwrap()
        .map(|x| x.unwrap().to_path_buf());
    let jpeg = glob(&get_image_glob(dir, "jpeg"))
        .unwrap()
        .map(|x| x.unwrap().to_path_buf());
    let gif = glob(&get_image_glob(dir, "gif"))
        .unwrap()
        .map(|x| x.unwrap().to_path_buf());
    let png = glob(&get_image_glob(dir, "png"))
        .unwrap()
        .map(|x| x.unwrap().to_path_buf());
    Box::new(jpg.chain(jpeg).chain(gif).chain(png)) as ImageIter
}
