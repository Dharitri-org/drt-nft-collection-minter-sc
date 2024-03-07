use nft_minter::common_storage::MoaxValuePaymentsVecPair;

dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait TokenBalanceModule {
    fn add_balance(&self, token: MoaxOrDctTokenIdentifier, amount: &BigUint) {
        self.balance_for_token(&token).update(|b| {
            *b += amount;
        });
        let _ = self.known_tokens().insert(token);
    }

    fn update_balance_from_results(&self, result: MoaxValuePaymentsVecPair<Self::Api>) {
        let (moax_value, other_payments) = result.into_tuple();

        if moax_value > 0 {
            self.add_balance(MoaxOrDctTokenIdentifier::moax(), &moax_value);
        }
        for p in &other_payments {
            self.add_balance(
                MoaxOrDctTokenIdentifier::dct(p.token_identifier),
                &p.amount,
            );
        }
    }

    #[view(getTokenBalances)]
    fn get_token_balances(
        &self,
    ) -> MultiValueEncoded<MultiValue2<MoaxOrDctTokenIdentifier, BigUint>> {
        let mut balances = MultiValueEncoded::new();

        for token_id in self.known_tokens().iter() {
            let balance_for_token = self.balance_for_token(&token_id).get();
            if balance_for_token > 0 {
                balances.push((token_id, balance_for_token).into());
            }
        }

        balances
    }

    #[storage_mapper("knownTokens")]
    fn known_tokens(&self) -> UnorderedSetMapper<MoaxOrDctTokenIdentifier>;

    #[storage_mapper("balanceForToken")]
    fn balance_for_token(&self, token_id: &MoaxOrDctTokenIdentifier)
        -> SingleValueMapper<BigUint>;
}
