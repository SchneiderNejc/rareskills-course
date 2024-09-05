If you only paid enough rent for half a year, your account would get erased after six months. 
If you paid for two years of rent in advance, the account would be “rent exempt.” 
The account would never have to pay rent again. 
Nowadays, all accounts are required to be rent-exempt; you cannot pay less than 2 years of rent.

Although rent is computed on a “per byte” basis, accounts with zero data are not free; Solana still has to index them and store metadata about them.

When accounts are initialized, the amount of rent needed is computed in the background; you don’t need to calculate the rent explicitly.

When we initialize an account, we cannot initialize more than 10,240 bytes in size.

When increasing the size of the account, be sure to set realloc::zero = false if you do not want the account data erased.

solana program show [programId]
-Find storage address under ProgramData Address
solana account <addr>
-View account size

The cost can be significantly reduced by writing raw Rust code instead of using the Anchor framework, but we don't recommend doing that until you fully understand all the security risks Anchor eliminates by default.