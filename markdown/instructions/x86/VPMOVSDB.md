> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVSDB

Copies a byte string from a source memory location to a destination memory location. The number of bytes to copy is determined by the value in the `rcx` register. The instruction increments or decrements the source and destination pointers based on the direction flag (DF).

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m1 | m1 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the `rcx` register to specify the count of bytes to be moved.

The instruction MUST be used with the `rsi` register as the source pointer and `rdi` register as the destination pointer. If the source and destination memory regions overlap, the behavior is undefined. The value of `rcx` is decremented after each byte transfer; the loop terminates when `rcx` reaches 0.