How does Anchor know an account is already initialized?
From Anchor’s perspective, if the account has a non-zero lamport balance OR the account is owned by the system program, then it is not initialized.
An account owned by the system program or with zero lamport balance can be initialized again.

Solana does not have an “initialized” flag or anything. Anchor will allow an initialize transaction to succeed if the owner is the system program or the lamport balance is zero.

Transferring ownership to the system program requires erasing the data in the account.

It is important that we erase the data using an UncheckedAccount as .realloc(0, false) is not a method available on a regular Account.

This operation will erase the account discriminator, so it will not be readable via Account anymore.


Summary
The init_if_needed macro can be convenient to avoid needing two transactions to interact with a new storage account. The Anchor framework blocks it by default to force us to consider the following possible undesirable situations:

If there is a method to reduce the lamport balance to zero or transfer ownership to the system program, then the account can be re-initialized. This may or may not be a problem depending on the business requirements.
If the program has both an init macro and a init_if_needed macro, the developer must ensure that having two codepaths doesn’t result in unexpected state.
Even after the data in an account is completely erased, the account is still initialized.
If the program has a function that “blindly” writes to an account, then data in that account could get overwritten. This usually requires loading in the account via AccountInfo or its alias UncheckedAccount.