> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XBEGIN

Starts a transactional memory block. The instruction saves the current state of the processor and checkpoints the register state. If a transaction is already in progress, XBEGIN is ignored. If a transaction fails, the processor rolls back the architectural state to the checkpoint created by XBEGIN and transfers control to the fallback address specified by the operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm32 | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

XBEGIN is only available in 64-bit mode. It is NOT supported in compatibility mode. The instruction requires the processor to support Intel TSX (Transactional Synchronization Extensions).

The fallback address provided as an immediate or register must be a valid memory address within the current execution context to avoid a General Protection fault (#GP) upon transaction abort. If the `RTM` (Restrictive Transactional Memory) feature is disabled in `CR4.RTM`, executing XBEGIN SHALL trigger an invalid opcode exception (#UD).