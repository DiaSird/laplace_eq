pub mod module;

pub const LIM: usize = 1000; // maximum repeated number
pub const NX: usize = 100; // x-direction splitted number
pub const NY: usize = 100; // y-direction splitted number

// file path
#[derive(Debug, Clone)]
pub struct Config {
    pub dir_path: String,
    pub file_path: String,
    pub dir_img: String,
    pub img_path: String,
    pub img_path2: String,
}

impl Config {
    #[allow(unused)]
    pub fn new(dir_path: &str) -> Self {
        Self {
            dir_path: format!("rst"),
            file_path: format!("{}/{}", dir_path, "rst_u.csv"),
            dir_img: format!("{}", dir_path),
            img_path: format!("{}/{}", dir_path, "2d_laplace.png"),
            img_path2: format!("{}/{}", dir_path, "2d_laplace2.png"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dir_path: "rst".into(),
            file_path: "rst/rst_u.csv".into(),
            dir_img: "rst".into(),
            img_path: "rst/2d_laplace.png".into(),
            img_path2: "rst/2d_laplace2.png".into(),
        }
    }
}
