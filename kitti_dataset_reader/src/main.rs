use clap::Parser;
use dora_node_api::arrow::datatypes::DataType;
use std::sync::Arc;
use std::io::prelude::*;
use image::io::Reader as ImageReader;
use dora_node_api::arrow::datatypes::Field;
use dora_node_api::arrow::array::{ArrayData, BufferBuilder};
use dora_node_api::arrow::array::FixedSizeListArray;
use dora_node_api::dora_core::config::DataId;
use dora_node_api::{DoraNode, Event};
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
    
    let output_id = DataId::from("stereo_image".to_owned());

    let (mut node, mut events) = DoraNode::init_from_env().unwrap();

    let dataset_path = args.dataset_path
            .canonicalize().unwrap();
    let workdir = dataset_path.parent().unwrap();

    let file = std::fs::File::open(&dataset_path).unwrap();
    let file = std::io::BufReader::new(file);

    let mut lines = file.lines().filter_map(|l| l.ok());

    loop {
        let event = match events.recv() {
            Some(input) => input,
            None => break,
        };

        match event {
            Event::Input {
                id,
                metadata,
                data: _
            } => match id.as_str() {
                "tick" => {
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
                        
                        let data_type = 
                        DataType::FixedSizeList(
                            Arc::new(Field::new("raw_data", DataType::UInt8, false)), 
                            r_image.as_raw().len() as _);
            
                        let mut buffer = BufferBuilder::<u8>::new(2*r_image.as_raw().len());
                        buffer.append_slice(r_image.as_raw());
                        buffer.append_slice(l_image.as_raw());
                        let buffer = buffer.finish();

                        let value_data = ArrayData::builder(DataType::UInt8)
                            .len(2*r_image.as_raw().len())
                            .add_buffer(buffer).build().unwrap();

                        let list_data = ArrayData::builder(data_type)
                            .len(2)
                            .add_child_data(value_data)
                            .build().unwrap();
                        let list_array = FixedSizeListArray::from(list_data);
                        
                        node.send_output(output_id.clone(), metadata.parameters, list_array).unwrap();
                    } else {
                        break;
                    }
                },
                other => eprintln!("Ignoring unexpected input `{other}`"),
            },
            Event::Stop => println!("Recived Manual stop"),
            other => eprintln!("Received unexpected input: {other:?}")
        }
    }
}
