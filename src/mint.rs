use crate::*;
#[near_bindgen]

impl Contract {
    #[payable]
    pub fn batch_mint(
        &mut self,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        num_to_mint: u64, 
        art_id:Vec<String>,                              // Num of Tokens to Mint
        amount: Option<U128>,                           // For NFT Price
        royalty: Option<HashMap<AccountId, u32>>,       // Royalties
        split_payment: Option<HashMap<AccountId, u32>>, // SplitPayments
    ) -> TokenSeriesJson {
        assert_at_least_one_yocto();
        assert!(num_to_mint > 0, "PLEASE ENTER NUM TO MINT  > 0 ");
        assert!(
            num_to_mint <= 125,
            "CANNOT MINT MORE THAN 125 TOKENS DUE TO GAS LIMITS"
        );

        assert!(
            env::is_valid_account_id(receiver_id.as_bytes()),
            "THE GIVE ACCOUNT ID IS INVALID"
        );
        if !self.whitelist.contains(&env::predecessor_account_id()){
            env::panic_str("ACCOUNT NOT WHITELISTED");
        }

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let mut total_perpetual = 0;
        let mut total_accounts = 0;
        let royalty_res: HashMap<AccountId, u32> = if let Some(royalty) = royalty {
            for (k, v) in royalty.iter() {
                if !env::is_valid_account_id(k.as_bytes()) {
                    env::panic_str("ACCOUNT ID NOT VALID");
                };
                total_perpetual += *v;
                total_accounts += 1;
            }
            royalty
        } else {
            HashMap::new()
        };

        let split_res: HashMap<AccountId, u32> = if let Some(split_payment) = split_payment {
            for (k, v) in split_payment.iter() {
                if !env::is_valid_account_id(k.as_bytes()) {
                    env::panic_str("ACCOUNT ID NOT VALID");
                };
                total_perpetual += *v;
                total_accounts += 1;
            }
            split_payment
        } else {
            HashMap::new()
        };

        assert!(
            total_accounts <= 10,
            "LIMIT EXCEEDED ONLY 10 ACCOOUNTS ARE ALLOWED"
        );

        assert!(
            total_perpetual <= 5000,
            " LIMIT EXCEEDED ONLY ALLOWED  50% FOR ROYALTIES AND SPLIT PAYMENTS",
        );

        let price: Option<u128> = if amount.is_some() {
            Some(amount.unwrap().0)
        } else {
            None
        };

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,

            // Price of NFT
            price,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty: royalty_res,

            splitpayments: split_res,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        let mut tokens: Vec<u64> = Vec::new();
        (0..num_to_mint).for_each(|i| {
            // let token_id = format!("{}" , self.token_metadata_by_id.len()+i) ;
            let token_id = self.tokens_minted + i;
            tokens.push(token_id);
            self.tokens_by_id.insert(&token_id.to_string(), &token);

            //insert the token ID and metadata
            self.token_metadata_by_id.insert(&token_id.to_string(), &metadata);

            //call the internal method for adding the token to the owner
            self.internal_add_token_to_owner(&token.owner_id, &token_id.to_string());

            // Construct the mint log as per the events standard.
            let nft_mint_log: EventLog = EventLog {
                // Standard name ("nep171").
                standard: NFT_STANDARD_NAME.to_string(),
                // Version of the standard ("nft-1.0.0").
                version: NFT_METADATA_SPEC.to_string(),
                // The data related with the event stored in a vector.
                event: EventLogVariant::NftMint(vec![NftMintLog {
                    // Owner of the token.
                    owner_id: token.owner_id.to_string(),
                    // Vector of token IDs that were minted.
                    token_ids: vec![token_id.to_string()],
                    // An optional memo to include.
                    memo: None,
                }]),
            };
            // Log the serialized json.
            env::log_str(&nft_mint_log.to_string());
        });

        self.tokens_minted += num_to_mint;

        let minted = self.tokens_minted - 1;

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

        TokenSeriesJson {
            token_id: minted.to_string(),
            metadata: metadata,
            owner_id: token.owner_id,
            token_ids: tokens,
            art_id
        }
    }



     #[payable]
    pub fn add_whitelist_address(&mut self, account_ids:Vec<AccountId>)  {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id.clone(),
            " UNAUTHORIZED - ONLY OWNER "
        );


        for i in account_ids.iter(){
            assert!(
                env::is_valid_account_id(i.as_bytes()),
                "Account ID is invalid"
            );
            self.whitelist.insert(&i);   
        }
            
        }



        #[payable]  
        pub fn remove_white_list(&mut self, account_id: AccountId) -> bool {
            assert_one_yocto();
            assert_eq!(
                env::predecessor_account_id(),
                self.owner_id.clone(),
                " UNAUTHORIZED - ONLY OWNER "
            );
            assert!(
                env::is_valid_account_id(account_id.as_bytes()),
                "The given account ID is invalid"
            );
            self.whitelist.remove(&account_id)
        }

     /// Returns `true` if the given  account ID is whitelisted.
     pub fn is_whitelisted(&self, account_id: AccountId) -> bool {
        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "The given account ID invalid"
        );
        self.whitelist.contains(&account_id)
    }

}
