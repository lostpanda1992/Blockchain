//httpresponse模块
use std::collections::HashMap;
use std::io::{Result, Write};
use std::mem::replace;

//Debug 打印调试信息 PartialEq成员可以与其他变量比较 clone本身可以克隆
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}", 
            &res1.version(), 
            &res1.status_code(), 
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str, 
        headers:Option<HashMap<&'a str, &'a str>>,
        body:Option<String>
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers{
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "ERROR".into(),
        };

        response.body = body;
        response
    }

    //接受write_stream参数 该参数要实现Write这个trait 
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        // 对HttpResponse克隆
        let res = self.clone();
        let response_string:String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    //实现一系列查看方法
    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    //对外查看接口 所以不能用Some(...) 要用String
    fn headers(&self) -> String{
        let map:HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string:String = "".into();
        for (key, value) in map.iter() {
            header_string = format!("{}{}: {}\r\n", header_string,key, value);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200",None,Some("xxx".into()));
        let response_expected = HttpResponse{
            version:"HTTP/1.1",
            status_code:"200",
            status_text:"OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404",None,Some("xxx".into()));
        let response_expected = HttpResponse{
            version:"HTTP/1.1",
            status_code:"404",
            status_text:"Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
}