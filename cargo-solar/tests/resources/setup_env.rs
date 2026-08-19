use mocked_up::file_system::TempEnv;

use crate::resources::copy_bin;

pub fn setup_env() -> TempEnv {
    let mut temp = TempEnv::new().unwrap();
    copy_bin(temp.env().path());
    temp
}
