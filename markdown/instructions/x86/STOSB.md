> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STOSB

Stores the byte in the `AL` register into the memory location pointed to by the `RDI` register, then increments or decrements `RDI` by 1 according to the Direction Flag (DF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | m1 |

DO NOT support LOCK

The instruction SHALL operate in 64-bit mode, 32-bit mode, and 16-bit mode. The behavior of the `RDI` register increment or decrement is strictly dependent on the state of the DF flag in the RFLAGS register.

If the instruction is used within a `REP` or `REPE`/`REPNE` prefix, the operation SHALL be repeated `RCX` times. Failure to initialize `RCX` or setting it to 0 SHALL result in the instruction being skipped or executing $2^{64}$ times depending on the processor implementation. Ensure that the memory region pointed to by `RDI` is writable to avoid a general protection fault (#GP) or page fault.