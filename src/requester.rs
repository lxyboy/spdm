use crate::protocol;

pub fn get_version() {
    use protocol::GET_VERSION;
    let buffer = [0, 1, 2, 3];
    let get_version = GET_VERSION::new_checked(buffer).unwrap();
}
