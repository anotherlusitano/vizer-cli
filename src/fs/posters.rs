// we could use a simpler code, like this:
// for media in medias {
//     let img = reqwest::get(media.poster_url).await?.bytes().await?;
//     let path_img = format!("{}/{}.jpg", vizer_temp, media.title);
//
//     fs::write(&path_img, img).unwrap();
//     posters_path.push(path_img);
// }
// return posters_path
//
// but its much slower
// if the user search for 'harry' it will get 200 results
// and that can take a whole minute to download everything
// and this form only take 20 or less seconds

use std::{
    env, error, fs,
    sync::{Arc, Mutex},
};

use futures::StreamExt;
use reqwest::Client;

use crate::TRANSLATION;

// this function basically downloads the image from each url
// and creates the image file in the vizer temporary directory
pub async fn get_posters_path(urls: Vec<String>) -> Result<Vec<String>, Box<dyn error::Error>> {
    let language = TRANSLATION.get().unwrap();

    let temp_dir = env::temp_dir();
    let posters_path = Arc::new(Mutex::new(vec![None; urls.len()]));

    let client = Client::builder().build()?;
    let fetches = futures::stream::iter(urls.into_iter().enumerate().map(|(index, url)| {
        let send_future = client.get(&url).send();
        let vizer_temp = format!("{}/vizer", temp_dir.display());
        let posters_path = Arc::clone(&posters_path);
        async move {
            match send_future.await {
                Ok(resp) => match resp.bytes().await {
                    Ok(img) => {
                        let img_id = url.split('/').last().unwrap().trim_end_matches(".jpg");

                        let path_img = format!("{}/{}.jpg", vizer_temp, img_id);

                        let msg = format!("{}, {}", language.msg_err, path_img);
                        if let Err(e) = fs::write(&path_img, img) {
                            println!("{}: {}", msg, e);
                        } else {
                            posters_path.lock().unwrap()[index] = Some(path_img.clone());
                        }
                    }
                    Err(_) => println!("{} {}", language.reading_err, url),
                },
                Err(_) => println!("{} {}", language.downloading_err, url),
            }
        }
    }))
    .buffer_unordered(100)
    .collect::<Vec<()>>();
    fetches.await;

    let posters_path = posters_path
        .lock()
        .unwrap()
        .iter()
        .filter_map(|path| path.clone())
        .collect();
    Ok(posters_path)
}
