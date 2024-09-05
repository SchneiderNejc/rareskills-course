Day22 is a theory. 

Solana does not have fallback or receive functions

It is not possible to use reentrancy locks to directly defend against read only reentrancy in Solana. 
The program must expose flag for readers to know if the data is reliable.

There are no custom modifiers in Rust.

LAMPORTS_PER_SOL is not defined in the Anchor Framework. It is available in the Solana web3 js library.