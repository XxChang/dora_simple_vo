#![warn(unsafe_op_in_unsafe_fn)]

use dora_operator_api::{register_operator, DoraOperator, Event, types::arrow::array::{FixedSizeListArray, PrimitiveArray} };
use dora_operator_api::types::arrow::datatypes::UInt8Type;
use opencv::{core::{CV_8U, Mat_AUTO_STEP, hconcat2, Mat, vconcat2}, highgui::WINDOW_AUTOSIZE};
use opencv::highgui::{named_window, imshow, wait_key};

register_operator!(PlotStereoImage);

#[derive(Debug)]
struct PlotStereoImage {
    image: Mat,
}

impl Default for PlotStereoImage {
    fn default() -> Self {
        named_window("image", WINDOW_AUTOSIZE).unwrap();
        PlotStereoImage {
            image: Mat::default(),
        }
    }
}

impl DoraOperator for PlotStereoImage {
    fn on_event(
            &mut self,
            event: &dora_operator_api::Event,
            _output_sender: &mut dora_operator_api::DoraOutputSender,
        ) -> Result<dora_operator_api::DoraStatus, String> {
        match event {
            Event::Input { id, data } => match *id {
                "stereo_image" => {
                    let data_inner: FixedSizeListArray = data.to_data().into();
                    let raw_data_r: PrimitiveArray<UInt8Type> = data_inner.value(0).to_data().into();
                    let raw_data_l: PrimitiveArray<UInt8Type> = data_inner.value(1).to_data().into();

                    let raw_data_r = raw_data_r.values().to_vec();
                    let raw_data_l: Vec<u8> = raw_data_l.values().to_vec();

                    let raw_data_r = unsafe { 
                        opencv::core::Mat::new_rows_cols_with_data(370, 1226, CV_8U, raw_data_r.as_ptr() as _, Mat_AUTO_STEP).unwrap()
                    };
                    let raw_data_l = unsafe { 
                        opencv::core::Mat::new_rows_cols_with_data(370, 1226, CV_8U, raw_data_l.as_ptr() as _, Mat_AUTO_STEP).unwrap()
                    };

                    vconcat2(&raw_data_l, &raw_data_r, &mut self.image).unwrap();

                    imshow("image", &self.image).unwrap();
                    if wait_key(1).unwrap() > 0 {
                        return Ok(dora_operator_api::DoraStatus::Stop);
                    }
                },
                other => eprintln!("ignoring unexpected input {other}"),
            },
            Event::Stop => {},
            Event::InputClosed { id } => {
                println!("input `{id}` was closed");
                return Ok(dora_operator_api::DoraStatus::Stop);
            }
            other => {
                println!("receive unknown event {other:?}");
            }
        }

        Ok(dora_operator_api::DoraStatus::Continue)
    }
}

