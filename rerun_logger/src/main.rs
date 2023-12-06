use dora_node_api::{DoraNode, Event};
use dora_node_api::arrow::array::{PrimitiveArray, StructArray, FixedSizeListArray};
use dora_node_api::arrow::datatypes::{UInt32Type, UInt8Type};
use image::GenericImage;

fn main() {
    let rec = rerun::RecordingStreamBuilder::new("dora_example_app").connect().unwrap();

    let (mut _node, mut events) = DoraNode::init_from_env().unwrap();

    while let Some(event) = events.recv() {
        match event {
            Event::Input { 
                id, 
                metadata: _, 
                data 
            } => match id.as_str() {
                "stereo_image" => {
                    let data_inner: StructArray = data.to_data().into();
                    let width: PrimitiveArray<UInt32Type> = data_inner.column_by_name("width").unwrap().to_data().into();
                    let height: PrimitiveArray<UInt32Type> = data_inner.column_by_name("height").unwrap().to_data().into();

                    let width_r: u32 = width.value(0).into();
                    let width_l: u32 = width.value(1).into();

                    let height_r: u32 = height.value(0).into();
                    let height_l: u32 = height.value(1).into();

                    let raw_data_list: FixedSizeListArray = data_inner.column_by_name("raw").unwrap().to_data().into();
                    let raw_data_r: PrimitiveArray<UInt8Type> = raw_data_list.value(0).to_data().into();
                    let raw_data_l: PrimitiveArray<UInt8Type> = raw_data_list.value(1).to_data().into();

                    let raw_data_r = raw_data_r.values().to_vec();
                    let raw_data_l: Vec<u8> = raw_data_l.values().to_vec();

                    let image_r = image::GrayImage::from_raw(width_r, height_r, raw_data_r).unwrap();
                    let image_l = image::GrayImage::from_raw(width_l, height_l, raw_data_l).unwrap();
                    
                    let (width, height) = image_l.dimensions();
                    let mut result = image::ImageBuffer::new(width, height * 2);

                    result.copy_from(&image_l, 0, 0).unwrap();
                    result.copy_from(&image_r, 0, height).unwrap();

                    rec.log("image", &rerun::Image::try_from(result).unwrap()).unwrap();
                },
                other => {
                    eprintln!("Ignoring unexpected input `{other}`");
                    break;
                }
            },
            Event::Stop => {
                println!("Received Manual stop");
                break;
            },
            _other => {
                eprintln!("Received Manual stop");
                break;
            }
        }
    }
}
