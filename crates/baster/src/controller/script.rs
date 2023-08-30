use crate::resp::Resp;

pub async fn init() -> Resp<String> {
    Resp::success(r#"echo "Hello, blave!""#.to_owned())
}
