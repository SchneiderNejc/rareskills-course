In Solana, all “storage variables” can be read by any program, but only its owner program can write to it.

The Solana runtime does not force us to use structs to store data. From Solana’s perspective, the account just holds a data blob. However, Rust has a lot of convenient libraries for turning structs into data blobs and vice versa, so structs are the convention.

Don’t forget to reset the validator if running the test multiple times
If we could reinitialize an account, that would be highly problematic since a user could wipe data from the system! 

