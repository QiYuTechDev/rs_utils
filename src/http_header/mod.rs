/// 解析 HTTP 请求里面的 `HOST` 字段
pub fn parse_and_get_http_host_field(data: &str) -> Option<String> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut req = httparse::Request::new(&mut headers);

    if req.parse(data.as_bytes()).is_ok() {
        // parse https CONNECT method
        if let Some(conn) = req.method {
            if conn == "CONNECT" {
                return match req.path {
                    None => None,
                    Some(s) => Some(String::from(s)),
                };
            }
        }

        // parse http request
        for h in req.headers.iter() {
            // 这儿的确需要 传递小写 的 host 比较
            // 有些 curl 实现 传递的就是 小写 的 host
            // eg:
            // host: security.debian.org
            if h.name.to_ascii_lowercase() != "host" {
                continue;
            }

            if let Some(t) = req.path {
                let port = match url::Url::parse(t) {
                    Ok(u) => match u.port_or_known_default() {
                        Some(p) => p,
                        None => return None,
                    },

                    Err(_) => return None,
                };

                return Some(format!("{}:{}", String::from_utf8_lossy(h.value), port));
            };
        }
    }

    None
}

/// 解析 http header 字段
pub fn parse_and_get_http_host_field_with_error<E>(data: &str, e: E) -> Result<String, E> {
    match parse_and_get_http_host_field(data) {
        Some(o) => Ok(o),
        None => Err(e),
    }
}
