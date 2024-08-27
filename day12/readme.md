In Anchor programs, you can access sysvars in two ways: either by using the anchor’s get method wrapper, or by treating it as an account in your #[Derive(Accounts)], using its public address.

Clock: Used for time-related operations like getting the current time or slot number.
EpochSchedule: Contains information about epoch scheduling, including the epoch for a particular slot.
Rent: Contains the rental rate and information like the minimum balance requirements to keep an account rent exempt.
Fees: Contains the fee calculator for the current slot. The fee calculator provides information on how many lamports are paid per signature in a Solana transaction.
EpochRewards: The EpochRewards sysvar holds a record of epoch rewards distribution in Solana, including block rewards and staking rewards.
RecentBlockhashes: Contains the active recent block hashes.
SlotHashes: Contains history of recent slot hashes.
SlotHistory: Holds an array of slots available during the most recent epoch in Solana, and it is updated every time a new slot is processed.
StakeHistory: maintains a record of stake activations and deactivations for the entire network on a per-epoch basis, which is updated at the beginning of each epoch.
Instructions: To get access to the serialized instructions that are part of the current transaction.
LastRestartSlot: Contains the slot number of the last restart (the last time Solana restarted ) or zero if none ever happened. If the Solana blockchain were to crash and restart, an application can use this information to determine if it should wait until things stabilize.

solana slot
A slot is a window of time (about 400ms) where a designated leader can produce a block.

solana epoch
is approximately two days long. SOL can only be staked or unstaked at the start of an epoch

Sysvars we didn't test due changing pub struct Initialize
StakeHistory, RecentBlockhashes, Fees, Instruction, LastRestartSlot

In the current version of Anchor, it is not feasible to access certain sysvars.
These sysvars include EpochRewards, SlotHistory, and SlotHashes.
When attempting to access these sysvars, it results to an error.

