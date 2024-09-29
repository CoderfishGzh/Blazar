use failure::Error;
use std::{fs, io::Read};

pub async fn run() -> Result<(), Error> {
    // 读取配置
    let mut file = fs::File::open("default.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // 服务启动
    // todo
    Ok(())
}
