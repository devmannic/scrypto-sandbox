mod lib2;

use scrypto::prelude::*;

blueprint! {
    struct Hello {
        // Define what resources and data will be managed by Hello components
        sample_vault: Vault,
        hello2: lib2::Hello2,
    }

    impl Hello {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new() -> Component {
            // Create a new token called "HelloToken," with a fixed supply of 1000, and put that supply into a bucket
            let my_bucket: Bucket = ResourceBuilder::new()
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .new_token_fixed(1000);

            let c2 = lib2::Hello2::new();
            let c1 = Self {
                sample_vault: Vault::with_bucket(my_bucket),
                hello2: c2.into(),
            }
            .instantiate();
            c1
        }

        // This is a method, because it needs a reference to self.  Methods can only be called on components
        pub fn free_token(&mut self) -> (Bucket, Bucket) {
            info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            let b1 = self.sample_vault.take(1);
            //let b2 = self.sample_vault.take(1);
            //let b2 = self.hello2.call::<Bucket>("free_token2", vec![]);
            let b2 = self.hello2.free_token2();
            (b1, b2)
        }
    }
}
