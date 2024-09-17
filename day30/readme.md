The close = signer macro specifies that the signer in the transaction will receive the rent that was set aside to pay for storage 

In version 0.25 of Anchor, the close sequence was different.

When Anchor initializes an account, it computes the discriminator and stores that in the first 8 bytes of the account. The account discriminator is the first 8 bytes of the SHA256 of the Rust identifier of the struct.

When a user asks the program to load an account via pub the_pda: Account<'info, ThePda>, the program will compute the first 8 bytes of the SHA256 of the ThePda identifier. Then it will load ThePda data and compare the discriminator stored there to the one it computed. If they do not match, then Anchor will not deserialize the account.

To close a program, as opposed to an account owned by it, we can use the command-line:
solana program close <address> --bypass warning