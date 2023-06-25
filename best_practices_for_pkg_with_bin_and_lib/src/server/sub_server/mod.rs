// define a server struct
#[derive(Debug, Default)]
pub struct Server {
    pub name: String,
    pub port: u16,
    pub ip: String,
}
impl Server {
    pub fn new() -> Server {
        Server {
            name: "Sub Server".to_string(),
            port: 8080,
            ip: "".to_string(),
        }
    }

}
