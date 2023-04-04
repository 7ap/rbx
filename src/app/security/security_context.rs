#[derive(Debug)]
#[repr(C)]
pub enum Identities {
    Anonymous = 0,
    LocalGUI = 1,
    GameScript = 2,
    RobloxGameScript = 3,
    CmdLine = 4,
    StudioPlugin = 5,
    COM = 6,
    WebService = 7,
    Replicator = 8,
    CountIdentities = 9,
}

#[derive(Debug)]
#[repr(C)]
pub enum Permissions {
    None = 0,
    Plugin = 1,
    RobloxPlace = 2,
    LocalUser = 3,
    WritePlayer = 4,
    RobloxScript = 5,
    RobloxEngine = 6,
    NotAccessible = 7,
}
