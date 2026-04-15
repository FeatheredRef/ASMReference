> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PHSUBD

Subtracts two packed signed dword values from the source and the top of the stack, then pushes the result onto the stack.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | rsp |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The instruction modifies the stack pointer (`rsp`), decrementing it by 4 bytes to accommodate the result. Ensure the stack is properly aligned to avoid potential performance degradation or faults depending on the operating environment. The result is stored as a signed i32.