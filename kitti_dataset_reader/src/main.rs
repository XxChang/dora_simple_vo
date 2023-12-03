use clap::Parser;
use std::io::prelude::*;
use image::io::Reader as ImageReader;

mod kitti_message;
use kitti_message::KittiMessage;

#[derive(Debug, clap::Parser)]
#[clap(version)]
struct Args {
    #[clap(long)]
    dataset_path: std::path::PathBuf    
}

fn main() {
    let args = Args::parse();
    
    let dataset_path = args.dataset_path
            .canonicalize().unwrap();
    let workdir = dataset_path.parent().unwrap();

    let file = std::fs::File::open(&dataset_path).unwrap();
    let file = std::io::BufReader::new(file);

    let mut lines = file.lines().filter_map(|l| l.ok());

    loop {
        if let (Some(r_camera), Some(l_camera)) = (lines.next(), lines.next()) {
            let r_camera = KittiMessage::from_string(r_camera);
            let l_camera = KittiMessage::from_string(l_camera);
            let r_image_path = workdir.join(&r_camera.raw_data_filename);
            let l_image_path = workdir.join(&l_camera.raw_data_filename);
            let r_image = ImageReader::open(r_image_path).unwrap().decode().unwrap().to_luma8();
            let l_image = ImageReader::open(l_image_path).unwrap().decode().unwrap().to_luma8();

            if r_camera.seq_num != l_camera.seq_num {
                panic!("please sync message reading");
            }

            println!("{r_camera:?}");
            println!("{l_camera:?}");
        } else {
            break;
        }
    }

    

}
