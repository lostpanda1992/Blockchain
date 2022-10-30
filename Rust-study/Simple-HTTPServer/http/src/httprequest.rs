//httprequest模块 请求模块
//Method实现三个trait get post uninitialized
//Method通过实现From<&str>方法来返回不同的Method变体
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

// 目的就一个 通过条件 获得不同的Method变体 http连接方法
impl From<&str> for Method {
    fn from(s:&str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

// 目的就一个 通过条件 获得不同的Version变体 http版本
impl From<&str> for Version {
    fn from(s:&str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _=> Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String), // 路径
}

#[derive(Debug)]
pub struct HttpRequest { // http请求的结构
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>, //headers是一个key value集合
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        //以下定义为初始化
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path(String::from(""));
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        for line in req.lines() {
            if line.contains("HTTP"){
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":"){
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {

            } else {
                parsed_body = line;
            }
        }

        HttpRequest { 
            method: parsed_method, 
            version: parsed_version, 
            resource: parsed_resource, 
            headers: parsed_headers, 
            body: parsed_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
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

//简单测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        //into()转换成Method可接受的变体类型
        // assert_eq!(Method::Get.into(), "GET");
        let m:Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        //into()转换成Method可接受的变体类型
        // assert_eq!(Version::V1_1.into(), "HTTP/1.1");
        let v:Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
}