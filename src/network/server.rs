use crate::app::reflection::DescribedBase;
use crate::app::util::{ProtectedString, SharedString};
use crate::base::rbx::signals::ScopedConnection;
use crate::base::rbx::{Atomic, Signal, TaskSchedulerJob};
use crate::stl::memory::{SharedPtr, WeakPtr};
use crate::stl::mutex::Mutex;
use crate::stl::set::Set;
use crate::stl::string::String as CxxString;
use crate::stl::unordered_map::UnorderedMap;
use crate::stl::vector::Vector;

use super::raknet::SystemAddress;

use super::{
    MegaReplicator, NetworkSettings, Peer, Players, Replicator, ServerReplicator, ServerSchemaInfo,
};

#[derive(Debug)]
#[repr(C)]
pub enum ReplicationMode {
    Client = 0,
    StudioAsClient = 1,
    StudioAsEditor = 2,
}

#[derive(Debug)]
#[repr(C)]
pub struct LegalScript {
    pub shared_source: SharedString,
    pub shared_bytecode: SharedString,
}

#[derive(Debug)]
#[repr(C)]
pub struct Server {
    _super0: DescribedBase,
    _super1: Peer,
    pub is_public: bool,
    pub outgoing_port: i32,
    pub next_free_peer_id: u32,
    pub players: SharedPtr<Players>,
    pub network_owner_job: SharedPtr<TaskSchedulerJob>, // SharedPtr<NetworkOwnerJob> ?
    pub romark_phase_tracking_job: SharedPtr<TaskSchedulerJob>, // SharedPtr<RomarkPhaseTrackingJob> ?
    pub server_replication_mode: ReplicationMode,
    pub mega_replicator: MegaReplicator,
    pub closing_connection: ScopedConnection,
    pub item_added_connection: ScopedConnection,
    pub workspace_loaded_connection: ScopedConnection,
    pub ip_cache: Vector<usize>, // Vector<SecurityJoinInfo> !
    pub is_player_authentication_required: bool,
    pub is_join_untrottled: bool,
    pub network_settings: *mut NetworkSettings,
    pub last_reported_physics_unthrottled_world_step_id: i32,
    pub last_reported_physics_wall_time: f64, // long double ?
    pub legal_scripts: Vector<LegalScript>,
    pub scripts_by_shared_source: UnorderedMap<SharedString, i32>,
    pub scripts_by_shared_bytecode: UnorderedMap<SharedString, i32>,
    pub server_replicator_by_address: UnorderedMap<SystemAddress, *mut Replicator>,
    pub place_authentication_result_cache_mutex: Mutex,
    pub place_authentication_result_cache: UnorderedMap<i64, i32>,
    pub server_replicator_by_player: UnorderedMap<i64, WeakPtr<ServerReplicator>>,
    pub aslr_pmc_hasher: SharedPtr<usize>, // SharedPtr<AslrPmcSharedHasher> !
    pub new_gui_count: Atomic<u32>,
    pub moved_gui_count: Atomic<u32>,
    pub target_player_count: Atomic<u32>,
    pub cached_network_schema_info: SharedPtr<ServerSchemaInfo>,
    pub dyn_lua_challenge: ProtectedString,
    pub dynamic_lua_challenge_seed: u32,
    pub dyn_ip_rate_scaling_mutex: Mutex,
    pub dyn_ip_rate_scaling: i32,
    pub last_dyn_ip_rate_decrement: f64, // long double ?
    pub user_score_decryption_keys: Vector<CxxString>,
    pub ext_anti_cheat_signal: Signal<fn(Vector<u8>, usize, *const CxxString)>, // ?
    pub used_tickets: Set<CxxString>,
    pub preused_tickets: Set<CxxString>,
    pub remote_event_monitor: SharedPtr<usize>, // SharedPtr<RemoteEventMonitor> !
}
