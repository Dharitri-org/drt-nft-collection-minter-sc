dharitri_sc::imports!();

use dharitri_sc_modules::pause;

use crate::common_storage::MoaxValuePaymentsVecPair;

pub mod nft_marketplace_proxy {
    dharitri_sc::imports!();

    #[dharitri_sc::proxy]
    pub trait NftMarketplaceProxy {
        #[endpoint(claimTokens)]
        fn claim_tokens(
            &self,
            claim_destination: ManagedAddress,
            token_nonce_pairs: MultiValueEncoded<MultiValue2<MoaxOrDctTokenIdentifier, u64>>,
        ) -> super::MoaxValuePaymentsVecPair<Self::Api>;
    }
}

#[dharitri_sc::module]
pub trait NftMarketplaceInteractorModule:
    crate::royalties::RoyaltiesModule + crate::admin_whitelist::AdminWhitelistModule + pause::PauseModule
{
    #[endpoint(claimRoyaltiesFromMarketplace)]
    fn claim_royalties_from_marketplace(
        &self,
        marketplace_address: ManagedAddress,
        tokens: MultiValueEncoded<MoaxOrDctTokenIdentifier>,
    ) {
        self.require_caller_is_admin();
        self.require_not_paused();

        let mut args = MultiValueEncoded::new();
        for token in tokens {
            args.push((token, 0).into());
        }

        let own_sc_address = self.blockchain().get_sc_address();
        let call_result: MoaxValuePaymentsVecPair<Self::Api> = self
            .nft_marketplace_proxy_builder(marketplace_address)
            .claim_tokens(own_sc_address, args)
            .execute_on_dest_context();

        let (moax_amount, other_payments) = call_result.into_tuple();
        if moax_amount > 0 {
            self.add_royalties(MoaxOrDctTokenIdentifier::moax(), moax_amount);
        }
        if !other_payments.is_empty() {
            self.add_royalties_multiple(&other_payments)
        }
    }

    #[proxy]
    fn nft_marketplace_proxy_builder(
        &self,
        sc_address: ManagedAddress,
    ) -> nft_marketplace_proxy::Proxy<Self::Api>;
}
