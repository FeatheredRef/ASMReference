> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STMXCSR

Stores the contents of the MXCSR register into a destination memory location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| MXCSR | m32 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support to be enabled.

The destination memory operand MUST be aligned on a 4-byte boundary; failure to do so may result in an alignment check exception if alignment checking is enabled. Ensure the destination is a dword to avoid memory corruption or exceptions.