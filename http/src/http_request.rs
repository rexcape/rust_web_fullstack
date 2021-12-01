use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> HttpMethod {
        match s {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            _ => HttpMethod::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for HttpVersion {
    fn from(s: &str) -> HttpVersion {
        match s {
            "HTTP/1.1" => HttpVersion::V1_1,
            "HTTP/2.0" => HttpVersion::V2_0,
            _ => HttpVersion::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub version: HttpVersion,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = HttpMethod::Uninitialized;
        let mut parsed_version = HttpVersion::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (k, v) = process_header_line(line);
                parsed_headers.insert(k, v);
            } else if line.len() == 0 {
                // Do nothing
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (HttpMethod, Resource, HttpVersion) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: HttpMethod = "GET".into();
        assert_eq!(m, HttpMethod::Get);
    }

    #[test]
    fn test_version_into() {
        let v: HttpVersion = "HTTP/1.1".into();
        assert_eq!(v, HttpVersion::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s:String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\nTest Http Body");
        let mut headers_excepted = HashMap::new();
        headers_excepted.insert("Host".into(), " localhost".into());
        headers_excepted.insert("Accept".into(), " */*".into());
        headers_excepted.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(HttpMethod::Get, req.method);
        assert_eq!(HttpVersion::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_excepted, req.headers);
        assert_eq!("Test Http Body".to_string(), req.msg_body);
    }
}
