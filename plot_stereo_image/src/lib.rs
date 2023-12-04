#![warn(unsafe_op_in_unsafe_fn)]

use dora_operator_api::{register_operator, DoraOperator, Event };

register_operator!(PlotStereoImage);

#[derive(Debug, Default)]
struct PlotStereoImage;

impl DoraOperator for PlotStereoImage {
    fn on_event(
            &mut self,
            event: &dora_operator_api::Event,
            _output_sender: &mut dora_operator_api::DoraOutputSender,
        ) -> Result<dora_operator_api::DoraStatus, String> {
        match event {
            Event::Input { id, data } => match *id {
                "stereo_image" => {
                    println!("len: {:?}", data);
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

