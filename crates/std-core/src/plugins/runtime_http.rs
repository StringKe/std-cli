use deno_error::JsErrorBox;
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedHttpUrl {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) path: String,
}

pub(crate) fn parse_http_url(url: &str) -> Result<ParsedHttpUrl, JsErrorBox> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| JsErrorBox::generic("only http:// URLs are supported"))?;
    let (authority, path) = match rest.split_once('/') {
        Some((authority, path)) => (authority, format!("/{path}")),
        None => (rest, "/".to_string()),
    };
    if authority.is_empty() {
        return Err(JsErrorBox::generic("URL host is empty"));
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) if !host.is_empty() => {
            let parsed_port = port
                .parse::<u16>()
                .map_err(|_| JsErrorBox::generic(format!("invalid URL port: {port}")))?;
            (host.to_string(), parsed_port)
        }
        _ => (authority.to_string(), 80),
    };
    Ok(ParsedHttpUrl { host, port, path })
}

pub(crate) fn normalize_network_hosts(hosts: &[String]) -> Vec<String> {
    let mut normalized = hosts
        .iter()
        .filter_map(|host| {
            let trimmed = host.trim();
            if trimmed.is_empty() {
                None
            } else if trimmed.contains(':') {
                Some(trimmed.to_string())
            } else {
                Some(format!("{trimmed}:80"))
            }
        })
        .collect::<Vec<_>>();
    normalized.sort();
    normalized.dedup();
    normalized
}

pub(crate) fn http_get(parsed: &ParsedHttpUrl) -> std::io::Result<String> {
    let mut stream = TcpStream::connect((parsed.host.as_str(), parsed.port))?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    write!(
        stream,
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        parsed.path, parsed.host
    )?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body.to_string())
        .unwrap_or(response))
}
