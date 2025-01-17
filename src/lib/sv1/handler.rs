use super::bitcoin::Template;
use bitcoincore_rpc_json::GetBlockTemplateResult;
use sv1_api::client_to_server::{Authorize, Configure, Submit};
use sv1_api::error::Error;
use sv1_api::server_to_client::VersionRollingParams;
use sv1_api::utils::{Extranonce, HexU32Be};
use sv1_api::Message;

#[derive(Clone)]
pub struct Sv1Handler {
    pub template: Template,
    pub is_authorized: bool,
}

impl Sv1Handler {
    pub fn new(
        bitcoin_network: String,
        solo_miner_signature: String,
        solo_miner_address: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            template: Template::new(bitcoin_network, solo_miner_signature, solo_miner_address)?,
            is_authorized: false,
        })
    }

    pub fn update_template(&mut self, gbt_result: GetBlockTemplateResult) {
        self.template.update(gbt_result);
    }
}

impl<'a> sv1_api::IsServer<'a> for Sv1Handler {
    fn handle_configure(
        &mut self,
        request: &Configure,
    ) -> (Option<VersionRollingParams>, Option<bool>) {
        tracing::info!("handling mining.configure");
        let version_rolling_mask = request
            .version_rolling_mask()
            .map(|mask| HexU32Be(mask & 0x1FFFE000));
        let version_rolling_min_bit = request.version_rolling_min_bit_count();

        (
            Some(sv1_api::server_to_client::VersionRollingParams::new(
                version_rolling_mask.clone().unwrap_or(HexU32Be(0)),
                version_rolling_min_bit.clone().unwrap_or(HexU32Be(0)),
            ).expect("Version mask invalid, automatic version mask selection not supported, please change it in carte::downstream_sv1::mod.rs")),
            Some(false),
        )
    }

    // dummy
    fn handle_subscribe(
        &self,
        _request: &sv1_api::client_to_server::Subscribe,
    ) -> Vec<(String, String)> {
        tracing::info!("handling mining.subscribe");

        let set_difficulty_sub = (
            "mining.set_difficulty".to_string(),
            "ae6812eb4cd7735a302a8a9dd95cf71f".to_string(),
        );
        let notify_sub = (
            "mining.notify".to_string(),
            "ae6812eb4cd7735a302a8a9dd95cf71f".to_string(),
        );

        vec![set_difficulty_sub, notify_sub]
    }

    fn handle_authorize(&self, request: &Authorize) -> bool {
        !self.is_authorized(&request.name)
    }

    fn handle_submit(&self, _request: &Submit<'a>) -> bool {
        tracing::info!("handling mining.submit");

        true
    }

    fn handle_extranonce_subscribe(&self) {
        todo!()
    }

    fn is_authorized(&self, _name: &str) -> bool {
        self.is_authorized
    }

    fn authorize(&mut self, _name: &str) {
        self.is_authorized = true;
    }

    // dummy
    fn set_extranonce1(&mut self, _extranonce1: Option<Extranonce<'a>>) -> Extranonce<'a> {
        let extranonce: Extranonce<'a> =
            Extranonce::try_from("000000000000000000000000000000010000000000000001")
                .expect("should always work");
        extranonce
    }

    fn extranonce1(&self) -> Extranonce<'a> {
        todo!()
    }

    // dummy
    fn set_extranonce2_size(&mut self, _extra_nonce2_size: Option<usize>) -> usize {
        8
    }

    fn extranonce2_size(&self) -> usize {
        todo!()
    }

    fn version_rolling_mask(&self) -> Option<HexU32Be> {
        todo!()
    }

    fn set_version_rolling_mask(&mut self, _mask: Option<HexU32Be>) {
        // todo!()
    }

    fn set_version_rolling_min_bit(&mut self, _mask: Option<HexU32Be>) {
        // todo!()
    }

    fn notify(&mut self) -> Result<Message, Error> {
        todo!()
    }
}
