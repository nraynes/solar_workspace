use mocked_up::file_system::TempEnv;

use crate::resources::copy_bin;

pub fn setup_env() -> TempEnv {
    let temp = TempEnv::new().unwrap();
    copy_bin(temp.root().path());
    temp
}
