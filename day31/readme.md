Solana will try to process the transactions simultaneously. However there is a problem if they are accessing the same storage. 

are imported via the anchor_lang::prelude::*;;

purpose of Account, UncheckedAccount, Signer, and Program are to perform some kind of a check on the account passed in before proceeding.

Account type will check that the owner of the account being loaded is actually owned by the program. If the owner does not match, then it won’t load.
If we add an init macro to Account, then it will try to transfer ownership from the system program to this program. 

UncheckedAccount is an alias for AccountInfo. This does not check for ownership, so care must be taken as it will accept arbitrary accounts.

Signer type will check that account signed the transaction; it checks that the signature matches the public key of the account.

Program signals to Anchor the account is an executable one, and you may issue to it a cross program invocation.

