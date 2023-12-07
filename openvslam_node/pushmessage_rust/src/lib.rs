use std::sync::Arc;

use dora_node_api::{
    Event, arrow::{array::{StructArray, PrimitiveArray, FixedSizeListArray, UInt32Array, ArrayRef, BufferBuilder, ArrayData}, datatypes::{UInt32Type, DataType, Field}},
    EventStream
};
use dora_node_api::arrow::datatypes::UInt8Type;
use eyre::bail;

#[cxx::bridge]
#[allow(clippy::needless_lifetimes)]
mod ffi {
    struct StereoImageInfo {
        width_r: u32,
        height_r: u32,
        width_l: u32,
        height_l: u32,
        raw_data_l: Vec<u8>,
        raw_data_r: Vec<u8>,
    } 

    // pub struct FrameInfo {
    //     width: u32,
    //     height: u32,
    //     raw_data: Vec<u8>,
    // }

    struct DoraNode {
        events: Box<Events>,
        send_output: Box<OutputSender>,
    }

    pub enum DoraEventType {
        Stop,
        Input,
        InputClosed,
        Error,
        Unknown,
        AllInputsClosed,
    }

    struct DoraResult {
        error: String,
    }

    extern "Rust" {
        type Events;
        type OutputSender;
        type DoraEvent;

        fn init_dora_node() -> Result<DoraNode>;

        fn next_event(inputs: &mut Box<Events>) -> Box<DoraEvent>;
        fn event_type(event: &Box<DoraEvent>) -> DoraEventType;

        fn get_pic_from_event(input: Box<DoraEvent>) -> Result<StereoImageInfo>;
        fn put_frame(sender: &mut Box<OutputSender>, id: String, width: u32, height: u32, data: &[u8]) -> DoraResult;
    }
}

fn init_dora_node() -> eyre::Result<ffi::DoraNode> {
    let (node, events) = dora_node_api::DoraNode::init_from_env()?;
    let events = Events(events);
    let send_output = OutputSender(node);

    Ok(ffi::DoraNode {
        events: Box::new(events),
        send_output: Box::new(send_output),
    })
}

pub struct Events(EventStream);

fn next_event(events: &mut Box<Events>) -> Box<DoraEvent> {
    Box::new(DoraEvent(events.0.recv()))
}

pub struct DoraEvent(Option<Event>);

fn event_type(event: &DoraEvent) -> ffi::DoraEventType {
    match &event.0 {
        Some(event) => match event {
            Event::Stop => ffi::DoraEventType::Stop,
            Event::Input { .. } => ffi::DoraEventType::Input,
            Event::InputClosed { .. } => ffi::DoraEventType::InputClosed,
            Event::Error(_) => ffi::DoraEventType::Error,
            _ => ffi::DoraEventType::Unknown,
        },
        None => ffi::DoraEventType::AllInputsClosed,
    }
}

pub struct OutputSender(dora_node_api::DoraNode);

fn get_pic_from_event(input: Box<DoraEvent>) -> eyre::Result<ffi::StereoImageInfo> {
    let Some(Event::Input { id, metadata: _, data }) = input.0 else {
        bail!("not an input event");
    };

    if id.as_str() != "stereo_image" {
        bail!("not an stereo_image message");
    }

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
    let raw_data_l = raw_data_l.values().to_vec();

    Ok(ffi::StereoImageInfo {
        width_r,
        height_r,
        width_l,
        height_l,
        raw_data_l: raw_data_r,
        raw_data_r: raw_data_l,
    })
}

fn put_frame(sender: &mut Box<OutputSender>, id: String, width: u32, height: u32, data: &[u8]) -> ffi::DoraResult {
    let width = Arc::new(UInt32Array::from(vec![width]));
    let height = Arc::new(UInt32Array::from(vec![height]));

    let data_type = DataType::FixedSizeList(
        Arc::new(Field::new("item", DataType::UInt8, false)), 
        data.len() as _
    );

    let mut buffer = BufferBuilder::<u8>::new(data.len());
    buffer.append_slice(data);
    let buffer = buffer.finish();

    let value_data = ArrayData::builder(DataType::UInt8)
        .len(data.len())
        .add_buffer(buffer).build();

    if let Some(e) = value_data.as_ref().err() {
        return ffi::DoraResult { error: format!("{e:?}")};
    }

    let value_data = value_data.unwrap();

    let list_data = ArrayData::builder(data_type.clone())
        .len(1)
        .add_child_data(value_data)
        .build();

    if let Some(e) = list_data.as_ref().err() {
        return ffi::DoraResult { error: format!("{e:?}")};
    }

    let list_data = list_data.unwrap();

    let list_array = Arc::new(FixedSizeListArray::from(list_data));
    let struct_array = StructArray::from(vec![
        (
            Arc::new(Field::new("width", DataType::UInt32, false)),
            width.clone() as ArrayRef,
        ),
        (
            Arc::new(Field::new("height", DataType::UInt32, false)),
            height.clone() as ArrayRef,
        ),
        (
            Arc::new(Field::new("raw", data_type, false)),
            list_array.clone() as ArrayRef,
        )
    ]);

    let result = sender
        .0
        .send_output(id.into(), Default::default(), struct_array);
    let error = match result {
        Ok(()) => String::new(),
        Err(err) => format!("{err:?}"),
    };
    ffi::DoraResult { error }
    
    // if let Some(value_data) = value_data.ok() {
    //     let list_array = Arc::new(FixedSizeListArray::from(value_data));
    //     let struct_array = StructArray::from(vec![
    //         (
    //             Arc::new(Field::new("width", DataType::UInt32, false)),
    //             width.clone() as ArrayRef,
    //         ),
    //         (
    //             Arc::new(Field::new("height", DataType::UInt32, false)),
    //             height.clone() as ArrayRef,
    //         ),
    //         (
    //             Arc::new(Field::new("raw", data_type, false)),
    //             list_array.clone() as ArrayRef,
    //         )
    //     ]);
        
    //     let result = sender
    //         .0
    //         .send_output(id.into(), Default::default(), struct_array);
    //     let error = match result {
    //         Ok(()) => String::new(),
    //         Err(err) => format!("{err:?}"),
    //     };
    //     ffi::DoraResult { error }
    // } else {
    //     return ffi::DoraResult { error: format!("ArrayData build failed") };
    // }
}
