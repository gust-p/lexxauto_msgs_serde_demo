use ::lexxauto_msgs_rs::std_srvs::{SetBoolRequest, SetBoolResponse};
use lexxauto_msgs_rs::geometry_msgs::Vector3;
use pyo3::{
    Bound, PyResult,
    types::{PyModule, PyModuleMethods},
};

#[pyo3::pyclass(name = "Vector3")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VectorClass(Vector3);

#[pyo3::pyclass(name = "SetBoolSrvResponse")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetBoolSrvResponse(SetBoolResponse);

#[pyo3::pyclass(name = "SetBoolSrvRequest")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetBoolSrvRequest(SetBoolRequest);

#[pyo3::pymethods]
impl VectorClass {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3 { x, y, z })
    }

    pub fn serialize(&self) -> Vec<u8> {
        println!("Serializing: {:?}", &self.0);
        cdr::serialize::<_, _, cdr::CdrLe>(&self.0, cdr::size::Infinite).unwrap()
    }

    #[staticmethod]
    pub fn deserialize(data: Vec<u8>) -> Self {
        let deserialized = cdr::deserialize::<Vector3>(&data).unwrap();
        Self(deserialized)
    }

    pub fn __repr__(&self) -> String {
        format!("Vector3(x={}, y={}, z={})", self.0.x, self.0.y, self.0.z)
    }
}

#[pyo3::pymethods]
impl SetBoolSrvResponse {
    #[new]
    pub fn new(success: bool, message: String) -> Self {
        Self(SetBoolResponse { success, message })
    }

    #[staticmethod]
    pub fn deserialize(data: Vec<u8>) -> Self {
        let deserialized = cdr::deserialize::<SetBoolResponse>(&data).unwrap();
        Self(deserialized)
    }

    #[getter]
    pub fn success(&self) -> bool {
        self.0.success
    }

    #[getter]
    pub fn message(&self) -> String {
        self.0.message.clone()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "SetBoolResponse(success={}, message='{}')",
            self.0.success, self.0.message
        )
    }
}

#[pyo3::pymethods]
impl SetBoolSrvRequest {
    #[new]
    pub fn new(data: bool) -> Self {
        Self(SetBoolRequest { data })
    }

    pub fn serialize(&self) -> Vec<u8> {
        println!("Serializing SetBoolRequest: {:?}", &self.0);
        cdr::serialize::<_, _, cdr::CdrLe>(&self.0, cdr::size::Infinite).unwrap()
    }

    #[getter]
    pub fn data(&self) -> bool {
        self.0.data
    }

    pub fn __repr__(&self) -> String {
        format!("SetBoolRequest(data={})", self.0.data)
    }
}

#[pyo3::pymodule]
fn zenoh_chatter_demo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VectorClass>()?;
    m.add_class::<SetBoolSrvRequest>()?;
    m.add_class::<SetBoolSrvResponse>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_serde() {
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

    #[test]
    fn test_setbool_serde() {
        let req = SetBoolSrvRequest::new(true);
        let serialized = req.serialize();

        // Note: You'd need to implement deserialize for request if needed
        println!("SetBool request serialized: {:?}", serialized);
    }
}
