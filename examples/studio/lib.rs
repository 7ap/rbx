mod console;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

use rbx::app::v8datamodel::DataModel;
use rbx::base::rbx::TaskScheduler;

fn main() -> Result<()> {
    console::attach()?;

    let task_scheduler = unsafe { TaskScheduler::get()?.as_mut() };
    log::info!("{:#?}", task_scheduler);

    let data_model = unsafe { DataModel::get()?.as_mut() };
    log::info!("{:#?}", data_model);

    while unsafe { !GetAsyncKeyState(VK_END.0 as i32) & 0x01 } == 0x01 {
        thread::sleep(Duration::from_millis(50));
    }

    console::detach()?;

    Ok(())
}

#[no_mangle]
unsafe extern "system" fn DllMain(module: HMODULE, reason: u32, _: usize) -> isize {
    if reason == 1 {
        thread::spawn(move || unsafe {
            match main() {
                Ok(_) => FreeLibraryAndExitThread(module, 0),
                Err(err) => {
                    log::error!("{:#?}", err);

                    while { !GetAsyncKeyState(VK_END.0 as i32) & 0x01 } == 0x01 {
                        thread::sleep(Duration::from_millis(50));
                    }

                    FreeLibraryAndExitThread(module, 1);
                }
            }
        });

        return 1;
    };

    0
}
