use solana_sbpf::{
    program::SBPFVersion,
    vm::Config,
};

pub fn v2_config() -> Config {
    Config {
        enabled_sbpf_versions: SBPFVersion::V2..=SBPFVersion::V2,
        ..Config::default()
    }
}
