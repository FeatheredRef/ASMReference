> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XRELEASE

The `XRELEASE` instruction is used to release a lock held by the current processor. It performs a store operation to memory that is ordered with respect to previous memory accesses, ensuring that all preceding stores are visible to other processors before the lock is released.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | m8 |
| reg | m16 |
| reg | m32 |
| reg | m64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The instruction requires the destination memory operand to be naturally aligned to the size of the access; otherwise, an alignment check exception may occur if alignment checking is enabled in the EFLAGS register.

To avoid memory ordering issues in multi-processor environments, `XRELEASE` SHOULD be used as the final operation in a critical section to ensure that all data modifications are globally visible before the lock variable is updated. Failure to use a release-semantic instruction can lead to race conditions where another processor acquires the lock before the previous owner's writes have propagated.