use ::lexxauto_msgs_rs::std_msgs::String as StdMsgsString;
use byteorder::LittleEndian;
use cdr::{CdrLe, Infinite};
use cdr_encoding::{from_bytes, to_vec, to_vec_with_size_hint};
use lexxauto_msgs_rs::geometry_msgs::Vector3;
use pyo3::{
    Bound, PyResult,
    types::{PyModule, PyModuleMethods},
};

#[pyo3::pyclass(name = "StdMsgsString")]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StdString(pub StdMsgsString);

#[pyo3::pyclass(name = "Vector3")]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VectorClass(Vector3);

#[pyo3::pymethods]
impl VectorClass {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3 { x, y, z })
    }

    pub fn serialize(&self) -> Vec<u8> {
        println!("Serializing: {:?}", &self.0);
        // use std::mem::size_of;
        // let size = size_of::<Vector3>();
        // let serialized = to_vec_with_size_hint::<Vector3, LittleEndian>(&self.0, size).unwrap();
        // println!("Serialized number of bytes: {}", serialized.len());
        // println!(
        //     "Serialized data: {:?}",
        //     &serialized
        //         .iter()
        //         .map(|b| format!("{:02X}", b))
        //         .collect::<Vec::<String>>()
        // );
        // serialized
        let buf = cdr::serialize::<_, _, cdr::CdrLe>(&self.0, cdr::size::Infinite).unwrap();
        buf
    }

    #[staticmethod]
    pub fn deserialize(data: Vec<u8>) -> Self {
        let (deserialized_msg, _consumed_byte_count) =
            from_bytes::<Vector3, LittleEndian>(&data).unwrap();
        println!("Deserialized number of bytes: {}", _consumed_byte_count);
        println!("Generated from bytes: {:?}", &deserialized_msg);
        Self(deserialized_msg)
    }

    pub fn __repr__(&self) -> String {
        format!("Vector3(x={}, y={}, z={})", self.0.x, self.0.y, self.0.z)
    }
}

#[pyo3::pymethods]
impl StdString {
    #[new]
    pub fn new(data: String) -> Self {
        StdString(StdMsgsString { data })
    }

    pub fn serialize(&self) -> Vec<u8> {
        println!("Serializing StdMsgsString: {:?}", &self.0);
        to_vec::<StdMsgsString, LittleEndian>(&self.0).unwrap()
    }

    pub fn serialize_cdr(&self) -> Vec<u8> {
        let encoded = cdr::serialize::<_, _, CdrLe>(&self, Infinite).unwrap();
        encoded
    }

    #[staticmethod]
    pub fn deserialize(data: Vec<u8>) -> Self {
        let (deserialized_msg, _consumed_byte_count) =
            from_bytes::<StdMsgsString, LittleEndian>(&data).unwrap();
        println!("Deserialized number of bytes: {}", _consumed_byte_count);
        println!("Generated from bytes: {:?}", &deserialized_msg);
        Self(deserialized_msg)
    }

    pub fn __repr__(&self) -> String {
        format!("StdMsgsString(data={})", self.0.data)
    }
}

#[pyo3::pymodule]
fn zenoh_chatter_demo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StdString>()?;
    m.add_class::<VectorClass>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        let vec = VectorClass(Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        });
        let serialized = vec.serialize();

        let deserialized = VectorClass::deserialize(serialized);

        assert_eq!(vec.0.x, deserialized.0.x);
        assert_eq!(vec.0.y, deserialized.0.y);
        assert_eq!(vec.0.z, deserialized.0.z);
    }
}
