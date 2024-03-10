use crate::TRANSLATION;
use std::{
    env,
    fs::{create_dir, remove_dir_all},
    path::Path,
};

pub fn create_temp_dir() {
    let tmp = env::temp_dir();
    let vizer_temp = format!("{}/vizer", tmp.display());
    if !Path::new(&vizer_temp).exists() {
        create_dir(vizer_temp).expect(TRANSLATION.create_temp_dir_expect);
    }
}

pub fn remove_temp_dir() {
    let temp_dir = env::temp_dir();
    let vizer_temp = format!("{}/vizer", temp_dir.display());
    remove_dir_all(vizer_temp).expect(TRANSLATION.remove_temp_dir_expect);
}
