pub fn create_multipart(boundary: &str, parts: Vec<(&str, &str, &[u8])>) -> Vec<u8> {
    let mut data = Vec::new();
    for (name, filename, content) in parts {
        data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        data.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                name, filename
            )
            .as_bytes(),
        );
        data.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
        data.extend_from_slice(content);
        data.extend_from_slice(b"\r\n");
    }
    data.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
    data
}
