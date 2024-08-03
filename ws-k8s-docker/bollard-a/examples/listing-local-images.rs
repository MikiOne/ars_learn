use bollard::Docker;
use bollard::image::ListImagesOptions;

use futures_util::future::FutureExt;

use std::default::Default;

// Use a connection function described above
// let docker = Docker::connect_...;

#[tokio::main]
async fn main() {
    use bollard::Docker;
    let docker = Docker::connect_with_local_defaults().unwrap();
    async move {
        let images = &docker.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await.unwrap();

        for image in images {
            println!("-> {:?}", image);
            println!("-------");
        }
    }.await;
}