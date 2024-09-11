When airdropping SOL, an additional confirmTransaction routine is needed because there seems to be race conditions in the runtime about when the SOL is actually airdropped and when the transaction is confirmed.

Should I use PDAs or Keypair accounts?
Once the account is initialized, they behave the same way, so practically there is not much difference.
Most applications use PDAs since they can be programatically addressed via the seeds parameter, but to access a keypair account you must know the address in advance. 
