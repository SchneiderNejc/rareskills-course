signer1 pubkey does not change but signer2 pubkey changes. This is because the wallet account assigned to the signer1 account (in the test) is from the provider, which is also the Solana wallet account in your local machine and the account assigned to signer2 is randomly generated each time you run anchor test —skip-local-validator.

solana address 
-retrieves your pubkey

