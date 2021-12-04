use scrypto::prelude::*;

blueprint! {
    struct Hello2 {
        // Define what resources and data will be managed by Hello components
        sample_vault: Vault
    }

    impl Hello2 {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new() -> Component {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let my_bucket: Bucket = ResourceBuilder::new()
                .metadata("name", "HelloToken2")
                .metadata("symbol", "HT2")
                .new_token_fixed(1000);

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                sample_vault: Vault::with_bucket(my_bucket)
            }
            .instantiate()
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token2(&mut self) -> Bucket {
            info!("My balance is: {} HelloToken2. Now giving away a token!", self.sample_vault.amount());
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.sample_vault.take(1)
        }
    }
}
