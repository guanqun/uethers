// TODO: split to error.rs
#[derive(Debug)]
pub enum UEthersError {
    Request(ureq::Error),
    Io(std::io::Error),
}

impl From<ureq::Error> for UEthersError {
    fn from(e: ureq::Error) -> Self {
        UEthersError::Request(e)
    }
}

impl From<std::io::Error> for UEthersError {
    fn from(e: std::io::Error) -> Self {
        UEthersError::Io(e)
    }
}
