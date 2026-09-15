use crate::error::EaseoError;
use super::model::{SEOContract, SEOContractConfig, ContractGenerator, ContractSite};

pub fn build_contract(config: &SEOContractConfig) -> Result<SEOContract, EaseoError> {
    Ok(SEOContract {
        contract_version: "1".to_string(),
        generator: ContractGenerator {
            name: "easeo".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        site: ContractSite {
            canonical_host: config.canonical_host.clone(),
            scheme: config.scheme.clone(),
        },
        defaults: config.defaults.clone(),
        rules: config.rules.clone(),
        exceptions: config.exceptions.clone(),
    })
}
