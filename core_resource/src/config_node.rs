use autocxx::prelude::*;
use std::collections::HashMap;

use crate::sdk;

macro_rules! config_nodes {
    (
        $enum_name: ident [ $(
            $type_name: ident $( : $rust_type: ty )?,
        )+ ],
        $type_mod_name: ident
    ) => {
    paste::paste! {
        #[derive(Debug)]
        pub enum $enum_name { $(
            $type_name $( ($rust_type) )?,
        )+ }

        impl $enum_name { $( $(
            pub fn [<as_ $type_name:snake>](self) -> Option<$rust_type> {
                match self {
                    $enum_name::$type_name(v) => Some(v),
                    $enum_name::None => None,
                    _ => panic!("Expected {}", stringify!($type_name))
                }
            }
        )? )+ }

        mod $type_mod_name {
            use super::*;
        $( $(
            #[allow(dead_code)]
            pub(super) type $type_name = $rust_type;
        )? )+
        }
    }
    };
}

config_nodes!(ConfigNode [
    Bool: bool,
    String: std::string::String,
    F64: f64,
    List: Vec<ConfigNode>,
    Dict: HashMap<String, ConfigNode>,
    None,
], config_type);

type ConfigNodePtr = UniquePtr<sdk::Config::Value_ValuePtr>;

pub(crate) unsafe fn read_config_node(node: ConfigNodePtr) -> ConfigNode {
    let node_type = sdk::config_node::get_type(node.as_ref().unwrap());
    let node_type = altv_sdk::ConfigValueType::try_from(node_type)
        .unwrap_or_else(|_| panic!("unknown config value type: {node_type}"));

    {
        use altv_sdk::ConfigValueType;
        use sdk::config_node::*;
        match node_type {
            ConfigValueType::Bool => ConfigNode::Bool(read_bool(node)),
            ConfigValueType::String => ConfigNode::String(read_string(node).to_string()),
            ConfigValueType::Number => ConfigNode::F64(read_f64(node)),
            ConfigValueType::List => {
                let raw = read_list(node);
                let vec = raw
                    .into_iter()
                    .map(|node| read_config_node(copy_value_ptr(node)))
                    .collect();
                ConfigNode::List(vec)
            }
            ConfigValueType::Dict => {
                let raw = read_dict(node);
                let entries = raw.into_iter().map(|pair| {
                    let key = read_dict_pair_key(pair).to_string();
                    let value = {
                        let raw = read_dict_pair_value(pair);
                        read_config_node(raw)
                    };
                    (key, value)
                });
                let dict = HashMap::from_iter(entries);
                ConfigNode::Dict(dict)
            }
            ConfigValueType::None => ConfigNode::None,
        }
    }
}

pub trait ConfigRootNodeStore {
    fn root_node(&self) -> &config_type::Dict;
}

pub trait Config: ConfigRootNodeStore {
    fn get_custom_key(&self, key: &str) -> Option<&ConfigNode> {
        self.root_node().get(key)
    }
}

/// See alt:V docs: [link](https://docs.altv.mp/articles/configs/server.html)
#[derive(Debug)]
pub struct ServerConfig {
    pub modules: Vec<String>,
    pub resources: Vec<String>,

    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub players: Option<i32>,
    pub password: Option<String>,
    pub announce: Option<bool>,
    pub token: Option<String>,
    pub gamemode: Option<String>,
    pub website: Option<String>,
    pub language: Option<String>,
    pub description: Option<String>,
    pub debug: Option<bool>,
    pub streaming_distance: Option<f64>,
    pub migration_distance: Option<f64>,
    pub timeout: Option<f64>,
    pub announce_retry_error_delay: Option<i32>,
    pub announce_retry_error_attempts: Option<i32>,
    pub duplicate_players: Option<i32>,
    pub map_bounds_min_x: Option<f64>,
    pub map_bounds_min_y: Option<f64>,
    pub map_bounds_max_x: Option<f64>,
    pub map_bounds_max_y: Option<f64>,
    pub map_cell_area_size: Option<f64>,
    pub col_shape_tick_rate: Option<i32>,
    pub log_streams: Option<Vec<String>>,
    pub entity_worker_count: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub connection_queue: Option<bool>,
    pub use_early_auth: Option<bool>,
    pub early_auth_url: Option<String>,
    pub use_cdn: Option<bool>,
    pub cdn_url: Option<String>,
    pub send_player_names: Option<bool>,
    pub spawn_after_connect: Option<bool>,
    pub world_profiler: Option<config_type::Dict>,
    pub js_module: Option<config_type::Dict>,
    pub csharp_module: Option<config_type::Dict>,
    pub voice: Option<config_type::Dict>,

    root_node: config_type::Dict,
}

impl ServerConfig {
    /// See alt:V docs: [link](https://docs.altv.mp/articles/configs/server.html)
    pub fn get() -> Self {
        let node_ptr = unsafe { sdk::ICore::GetServerConfig() };
        let config = unsafe { read_config_node(node_ptr) };
        let mut config = config.as_dict().unwrap_or_else(|| {
            panic!("Server config node must be of type dict");
        });

        Self {
            modules: get_string_list_required(config.remove("modules").unwrap()),
            resources: get_string_list_required(config.remove("resources").unwrap()),
            name: get_string(config.remove("name")),
            host: get_string(config.remove("host")),
            port: get_i32(config.remove("port")),
            players: get_i32(config.remove("players")),
            password: get_string(config.remove("password")),
            announce: get_bool(config.remove("announce")),
            announce_retry_error_attempts: get_i32(config.remove("announceRetryErrorAttempts")),
            announce_retry_error_delay: get_i32(config.remove("announceRetryErrorDelay")),
            token: get_string(config.remove("token")),
            gamemode: get_string(config.remove("gamemode")),
            website: get_string(config.remove("website")),
            language: get_string(config.remove("language")),
            description: get_string(config.remove("description")),
            debug: get_bool(config.remove("debug")),
            streaming_distance: get_f64(config.remove("streamingDistance")),
            migration_distance: get_f64(config.remove("migrationDistance")),
            timeout: get_f64(config.remove("timeout")),
            duplicate_players: get_i32(config.remove("duplicatePlayers")),
            map_bounds_min_x: get_f64(config.remove("mapBoundsMinX")),
            map_bounds_min_y: get_f64(config.remove("mapBoundsMinY")),
            map_bounds_max_x: get_f64(config.remove("mapBoundsMaxX")),
            map_bounds_max_y: get_f64(config.remove("mapBoundsMaxY")),
            map_cell_area_size: get_f64(config.remove("mapCellAreaSize")),
            col_shape_tick_rate: get_i32(config.remove("colShapeTickRate")),
            log_streams: get_string_list(config.remove("logStreams")),
            entity_worker_count: get_i32(config.remove("entityWorkerCount")),
            tags: get_string_list(config.remove("tags")),
            connection_queue: get_bool(config.remove("connectionQueue")),
            use_early_auth: get_bool(config.remove("useEarlyAuth")),
            early_auth_url: get_string(config.remove("earlyAuthUrl")),
            use_cdn: get_bool(config.remove("useCdn")),
            cdn_url: get_string(config.remove("cdnUrl")),
            send_player_names: get_bool(config.remove("sendPlayerNames")),
            spawn_after_connect: get_bool(config.remove("spawnAfterConnect")),
            world_profiler: get_dict(config.remove("worldProfiler")),
            js_module: get_dict(config.remove("js-module")),
            csharp_module: get_dict(config.remove("csharp-module")),
            voice: get_dict(config.remove("voice")),

            root_node: config,
        }
    }
}

impl ConfigRootNodeStore for ServerConfig {
    fn root_node(&self) -> &config_type::Dict {
        &self.root_node
    }
}

impl Config for ServerConfig {}

/// See alt:V docs: [link](https://docs.altv.mp/articles/configs/resource.html)
#[derive(Debug)]
pub struct ResourceConfig {
    root_node: config_type::Dict,
}

impl ResourceConfig {
    /// See alt:V docs: [link](https://docs.altv.mp/articles/configs/resource.html)
    pub(crate) fn new(node_ptr: ConfigNodePtr) -> Self {
        let config = unsafe { read_config_node(node_ptr) };
        let config = config.as_dict().unwrap_or_else(|| {
            panic!("Resource config node must be of type dict");
        });

        Self { root_node: config }
    }
}

impl ConfigRootNodeStore for ResourceConfig {
    fn root_node(&self) -> &config_type::Dict {
        &self.root_node
    }
}

impl Config for ResourceConfig {}

fn get_string_list_required(node: ConfigNode) -> Vec<String> {
    let list = node.as_list().unwrap();
    list.into_iter().map(|v| v.as_string().unwrap()).collect()
}

fn get_string_list(node: Option<ConfigNode>) -> Option<Vec<String>> {
    let Some(list) = node else {
        return None;
    };

    let list = list.as_list().unwrap();
    Some(list.into_iter().map(|v| v.as_string().unwrap()).collect())
}

fn get_string(node: Option<ConfigNode>) -> Option<String> {
    node.map(|v| v.as_string().unwrap())
}

fn get_i32(node: Option<ConfigNode>) -> Option<i32> {
    node.map(|v| v.as_f64().unwrap() as i32)
}

fn get_f64(node: Option<ConfigNode>) -> Option<f64> {
    node.map(|v| v.as_f64().unwrap())
}

fn get_bool(node: Option<ConfigNode>) -> Option<bool> {
    node.map(|v| v.as_bool().unwrap())
}

fn get_dict(node: Option<ConfigNode>) -> Option<config_type::Dict> {
    node.map(|v| v.as_dict().unwrap())
}
