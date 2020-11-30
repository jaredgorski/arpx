#[derive(Debug)]
pub struct UplinkMessage {
    pub cmd: String,
    pub action: String,
    pub parameters: Vec<String>,
    pub pid: String,
    pub process_name: String,
}
