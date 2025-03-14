pub struct Request {
    path: String,
    query: Option<String>,
    method: super::method::Method,
}