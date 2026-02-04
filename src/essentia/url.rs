#[derive(Debug)]
pub struct Url {
    pub scheme:   String,
    pub hostname: Option<String>,
    pub port:     Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub path:     String,
}

impl Url {
    pub fn parse(url: &str) -> Result<Self, &'static str> {
        let (scheme, rest) = url.split_once("://").ok_or("Invalid scheme")?;
        let scheme = scheme.to_string();
        let mut rest = rest;
        let auth = if let Some(at) = rest.find('@') {
            let auth_part = &rest[..at];
            rest = &rest[at + 1..];
            if let Some(colon) = auth_part.find(':') {
                Some((
                    auth_part[..colon].to_string(),
                    auth_part[colon + 1..].to_string(),
                ))
            } else {
                Some((auth_part.to_string(), String::new()))
            }
        } else {
            None
        };
        let (host_port, path_part) = rest.split_once('/').unwrap_or((rest, ""));
        let path = if path_part.is_empty() {
            String::from("/")
        } else {
            ["/", path_part].concat()
        };
        let (hostname, port) = if let Some(colon) = host_port.find(':') {
            (
                Some(host_port[..colon].to_string()),
                Some(host_port[colon + 1..].parse().map_err(|_| "Invalid port")?),
            )
        } else {
            (Some(host_port.to_string()), None)
        };
        Ok(Url {
            scheme,
            hostname,
            port,
            username: auth.as_ref().map(|(u, _)| u.clone()),
            password: auth.as_ref().map(|(_, p)| p.clone()),
            path,
        })
    }
}
