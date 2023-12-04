
use dora_node_api::{DoraNode, Event};
use opencv::{core::{CV_8U, Mat_AUTO_STEP, Mat, vconcat2}, highgui::WINDOW_AUTOSIZE};
use opencv::highgui::{named_window, imshow, wait_key};
use dora_node_api::arrow::array::{PrimitiveArray, StructArray, FixedSizeListArray};
use dora_node_api::arrow::datatypes::{UInt32Type, UInt8Type};

fn main() {
    let (mut _node, mut events) = DoraNode::init_from_env().unwrap();

    named_window("image", WINDOW_AUTOSIZE).unwrap();

    let mut image = Mat::default();

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

                    let raw_data_r = unsafe { 
                        opencv::core::Mat::new_rows_cols_with_data(height_r as _, width_r as _, CV_8U, raw_data_r.as_ptr() as _, Mat_AUTO_STEP).unwrap()
                    };
                    let raw_data_l = unsafe { 
                        opencv::core::Mat::new_rows_cols_with_data(height_l as _, width_l as _, CV_8U, raw_data_l.as_ptr() as _, Mat_AUTO_STEP).unwrap()
                    };

                    vconcat2(&raw_data_l, &raw_data_r, &mut image).unwrap();

                    imshow("image", &mut image).unwrap();
                    if wait_key(1).unwrap() > 0 {
                        break;
                    }
                },
                other => { 
                    eprintln!("Ignoring ubexpected input `{other}`");
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

    
    opencv::highgui::destroy_all_windows().unwrap();
}
