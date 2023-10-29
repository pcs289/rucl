use crate::error::Result;
use crate::Params;

#[derive(Debug)]
pub enum HttpVerb {
    HEAD,
    GET,
    POST,
}

pub async fn build_request(params: Params) -> Result<String> {
    let path = params.url.path();
    let use_get = params.get;
    let use_head = params.head;
    let data = params.data;
    let form = params.form;

    let verb = match (&data, &form, use_get, use_head) {
        // Whenever use_get => use GET
        (_, _, _, true) => HttpVerb::HEAD,
        (_, _, true, _) => HttpVerb::GET,
        // If form or data is set => use POST
        (Some(_), Some(_), _, _) | (None, Some(_), _, _) | (Some(_), None, _, _) => HttpVerb::POST,
        // By default => use GET
        _ => HttpVerb::GET,
    };

    let request = match &verb {
        HttpVerb::HEAD | HttpVerb::GET => format!("{:?} {} HTTP/1.0\r\n\r\n", verb, path),
        HttpVerb::POST => format!(
            "{:?} {} HTTP/1.0\r\n\r\n{}",
            verb,
            path,
            data.unwrap_or_default()
        ),
    };

    log::error!("{}", request);
    Ok(request)
}
