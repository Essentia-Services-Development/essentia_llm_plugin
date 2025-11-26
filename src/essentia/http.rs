use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

pub struct Response {
    pub status:  u16,
    pub headers: HashMap<String, String>,
    pub body:    Vec<u8>,
}

pub fn get(url: &str) -> Result<Response, &'static str> {
    let url = crate::essentia::url::Url::parse(url)?;
    match url.scheme.as_str() {
        "http" => get_http(&url),
        "https" => get_https(&url),
        _ => Err("Unsupported scheme"),
    }
}

fn get_http(url: &crate::essentia::url::Url) -> Result<Response, &'static str> {
    let host = url.hostname.clone().ok_or("No hostname")?;
    let port = url.port.unwrap_or(80);
    let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|_| "Connect failed")?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path, host
    );
    stream.write_all(request.as_bytes()).map_err(|_| "Write failed")?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).map_err(|_| "Read failed")?;
    parse_response(&response)
}

fn get_https(url: &crate::essentia::url::Url) -> Result<Response, &'static str> {
    let host = url.hostname.clone().ok_or("No hostname")?;
    let port = url.port.unwrap_or(443);
    let mut tls_stream = crate::essentia::tls::tls_connect(&host, port)?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path, host
    );
    tls_stream.write(request.as_bytes())?;
    let mut response = Vec::new();
    let mut buffer = [0u8; 1024];
    loop {
        let n = tls_stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&buffer[..n]);
    }
    parse_response(&response)
}

pub fn post(url: &str, body: &str) -> Result<Response, &'static str> {
    let url = crate::essentia::url::Url::parse(url)?;
    match url.scheme.as_str() {
        "http" => post_http(&url, body),
        "https" => post_https(&url, body),
        _ => Err("Unsupported scheme"),
    }
}

fn post_http(url: &crate::essentia::url::Url, body: &str) -> Result<Response, &'static str> {
    let host = url.hostname.clone().ok_or("No hostname")?;
    let port = url.port.unwrap_or(80);
    let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|_| "Connect failed")?;
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Length: {}\r\nContent-Type: \
         application/json\r\nConnection: close\r\n\r\n{}",
        url.path,
        host,
        body.len(),
        body
    );
    stream.write_all(request.as_bytes()).map_err(|_| "Write failed")?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).map_err(|_| "Read failed")?;
    parse_response(&response)
}

fn post_https(url: &crate::essentia::url::Url, body: &str) -> Result<Response, &'static str> {
    let host = url.hostname.clone().ok_or("No hostname")?;
    let port = url.port.unwrap_or(443);
    let mut tls_stream = crate::essentia::tls::tls_connect(&host, port)?;
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Length: {}\r\nContent-Type: \
         application/json\r\nConnection: close\r\n\r\n{}",
        url.path,
        host,
        body.len(),
        body
    );
    tls_stream.write(request.as_bytes())?;
    let mut response = Vec::new();
    let mut buffer = [0u8; 1024];
    loop {
        let n = tls_stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&buffer[..n]);
    }
    parse_response(&response)
}

#[allow(unused)]
pub fn post_with_auth(url: &str, auth: &str, body: &str) -> Result<Response, &'static str> {
    let url = crate::essentia::url::Url::parse(url)?;
    match url.scheme.as_str() {
        "http" => post_with_auth_http(&url, auth, body),
        "https" => post_with_auth_https(&url, auth, body),
        _ => Err("Unsupported scheme"),
    }
}

#[allow(unused)]
fn post_with_auth_http(
    url: &crate::essentia::url::Url, auth: &str, body: &str,
) -> Result<Response, &'static str> {
    let host = url.hostname.clone().ok_or("No hostname")?;
    let port = url.port.unwrap_or(80);
    let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|_| "Connect failed")?;
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nAuthorization: {}\r\nContent-Length: {}\r\nContent-Type: \
         application/json\r\nConnection: close\r\n\r\n{}",
        url.path,
        host,
        auth,
        body.len(),
        body
    );
    stream.write_all(request.as_bytes()).map_err(|_| "Write failed")?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).map_err(|_| "Read failed")?;
    parse_response(&response)
}

#[allow(unused)]
fn post_with_auth_https(
    url: &crate::essentia::url::Url, auth: &str, body: &str,
) -> Result<Response, &'static str> {
    let host = url.hostname.clone().ok_or("No hostname")?;
    let port = url.port.unwrap_or(443);
    let mut tls_stream = crate::essentia::tls::tls_connect(&host, port)?;
    let request = format!(
        "POST {} HTTP/1.1\r\nHost: {}\r\nAuthorization: {}\r\nContent-Length: {}\r\nContent-Type: \
         application/json\r\nConnection: close\r\n\r\n{}",
        url.path,
        host,
        auth,
        body.len(),
        body
    );
    tls_stream.write(request.as_bytes())?;
    let mut response = Vec::new();
    let mut buffer = [0u8; 1024];
    loop {
        let n = tls_stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&buffer[..n]);
    }
    parse_response(&response)
}

fn parse_response(data: &[u8]) -> Result<Response, &'static str> {
    let text = std::str::from_utf8(data).map_err(|_| "Invalid UTF8")?;
    let mut lines = text.lines();
    let status_line = lines.next().ok_or("No status line")?;
    let status: u16 = status_line
        .split_whitespace()
        .nth(1)
        .ok_or("No status")?
        .parse()
        .map_err(|_| "Invalid status")?;
    let mut headers = HashMap::new();
    let mut body_start = 0;
    for (i, line) in lines.enumerate() {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }
        if let Some(colon) = line.find(':') {
            let key = line[..colon].trim().to_string();
            let value = line[colon + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }
    let body = text.lines().skip(body_start).collect::<Vec<_>>().join("\n").into_bytes();
    Ok(Response { status, headers, body })
}
