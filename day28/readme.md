When passing a u32 value or smaller to Rust, we do not need to use a Javascript bignumber.

The total size of a Solana transaction cannot exceed 1232 bytes.

To determine if an account needs to be initialized, we check if it has zero lamports or is owned by the system program

We can see how many transactions it took by directing the logs into the file and counting the number of transactions that happened:

solana logs > logs.txt
# run `anchor deploy` in another shell
grep "Transaction executed" logs.txt | wc -l
This will roughly match what temporarily appears after the anchor test or anchor deploy command