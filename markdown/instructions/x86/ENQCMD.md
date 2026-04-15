> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENQCMD

Enqueues a submission entry into a submission queue. It writes a 64-bit value to a memory location and increments the queue tail pointer.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m8 |

DO NOT support LOCK

The instruction SHALL only be used in 64-bit mode. It is specifically designed for use with Intel® Data Streaming Accelerator (DSA). The memory operand MUST be aligned to an 8-byte boundary to avoid alignment check exceptions.

The destination memory operand MUST point to a valid DSA submission queue tail pointer. Failure to provide a correctly aligned and mapped memory region WILL result in a general-protection fault (#GP). This instruction is intended for use in a producer-consumer model where the hardware consumes the entries; therefore, software MUST ensure the queue is not full before execution to avoid data loss.