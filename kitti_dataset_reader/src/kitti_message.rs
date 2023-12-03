use serde::{Deserialize, de::Visitor};
use nalgebra::{geometry::Isometry3, Vector3, Matrix3};

#[derive(Debug)]
pub struct KittiMessage {
    pub message_type: String,
    pub topic: String,
    pub frame_id: String,
    pub seq_num: i32,
    pub timestamp: f64,
    pub offset: Isometry3<f32>,
    pub odom: Option<Isometry3<f32>>,
    pub imu: Option<Isometry3<f32>>,
    pub depth_scale: f32,
    pub raw_data_filename: String,
    pub camera_matrix: Matrix3<f32>,
}

impl<'de> Deserialize<'de> for KittiMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        struct KittiMessageVisitor;

        impl<'de> Visitor<'de> for KittiMessageVisitor {
            type Value = KittiMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct KittiMessage")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>, {
                let message_type = seq.next_element()?.unwrap();
                let topic = seq.next_element()?.unwrap();
                let frame_id = seq.next_element()?.unwrap();
                let seq_num = seq.next_element()?.unwrap();
                let timestamp = seq.next_element()?.unwrap();

                let pt = Vector3::new(
                    seq.next_element()?.unwrap(),
                    seq.next_element()?.unwrap(),
                    seq.next_element()?.unwrap()
                );
                let vec = Vector3::new(
                    seq.next_element()?.unwrap(),
                    seq.next_element()?.unwrap(),
                    seq.next_element()?.unwrap(),
                );
                let offset = Isometry3::new(pt, vec);

                let has_odom: i32 = seq.next_element()?.unwrap();
                let odom = if has_odom == 1 {
                    let pt = Vector3::new(
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap()
                    );
                    let vec = Vector3::new(
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap(),
                    );
                    Some(Isometry3::new(pt, vec))
                } else {
                    None
                };

                let has_imu: i32 = seq.next_element()?.unwrap();
                let imu = if has_imu == 1 {
                    let pt = Vector3::new(
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap()
                    );
                    let vec = Vector3::new(
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap(),
                        seq.next_element()?.unwrap(),
                    );
                    Some(Isometry3::new(pt, vec))
                } else {
                    None
                };

                let depth_scale = seq.next_element()?.unwrap();
                let raw_data_filename = seq.next_element()?.unwrap();

                let camera_matrix = Matrix3::new(
                    seq.next_element()?.unwrap(), seq.next_element()?.unwrap(), seq.next_element()?.unwrap(),
                    seq.next_element()?.unwrap(), seq.next_element()?.unwrap(), seq.next_element()?.unwrap(),
                    0.0f32, 0.0f32, 1.0f32
                );

                Ok(
                    KittiMessage {
                        message_type,
                        topic,
                        frame_id,
                        seq_num,
                        timestamp,
                        offset,
                        odom,
                        imu,
                        depth_scale,
                        raw_data_filename,
                        camera_matrix,
                   }
                )
            }
        }

        deserializer.deserialize_seq(KittiMessageVisitor)
    }
}

impl KittiMessage {
    pub fn from_string(message: String) -> KittiMessage {
        let message_collect: Vec<&str> = message.split_whitespace().collect();
        let message = message_collect.join(" ");
        let message: KittiMessage = csv::ReaderBuilder::new().has_headers(false)
            .delimiter(b' ').from_reader(message.as_bytes()).deserialize().next().unwrap().unwrap();
        message
    }
}
