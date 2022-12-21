pub fn generate_string(string: &str) -> Vec<u8> {
    let mut result_bytes: Vec<u8> = Vec::new();
    result_bytes.extend([116]); // JAVA_TC_STRING
    let byte_len = string.len() as u16;
    result_bytes.extend(byte_len.to_be_bytes());
    result_bytes.extend(string.as_bytes());
    result_bytes
}
