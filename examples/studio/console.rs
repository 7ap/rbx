use anyhow::{anyhow, Result};
use windows::Win32::System::Console::*;

pub fn attach() -> Result<()> {
    env_logger::init();

    if !unsafe { AllocConsole().into() } {
        return Err(anyhow!("failed to allocate console"));
    }

    Ok(())
}

pub fn detach() -> Result<()> {
    if !unsafe { FreeConsole().into() } {
        return Err(anyhow!("failed to free console"));
    }

    Ok(())
}
