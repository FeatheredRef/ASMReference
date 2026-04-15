> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EDBGWR]

Enter Cache Locking State. This instruction is used to enter a cache-locking state, which is a feature of Intel Software Guard Extensions (SGX). It attempts to lock a cache line in the cache to prevent it from being evicted, ensuring that subsequent accesses to that memory region are performed at cache speeds without the risk of cache misses.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| imm | #I |
| mN | #I |

DO NOT support LOCK

The instruction is only available when the processor is operating in SGX-enabled mode. It SHALL be executed within an enclave; execution outside of an enclave results in a general protection fault (#GP).

The instruction requires that the target memory address be aligned to a cache line boundary. Failure to provide an aligned address MAY result in an undefined behavior or a fault. The instruction SHALL only be used in conjunction with the corresponding `EXCLS` instruction to manage the cache-locking lifecycle. Ensure the processor supports the specific SGX feature set including cache-locking, otherwise the instruction WILL trigger an invalid opcode exception (#UD).