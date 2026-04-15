> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNOP

The instruction performs no operation. It is used to provide a padding or to ensure the FPU is in a known state.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction is available in both 64-bit and compatibility mode. It does not affect any architectural registers or flags.

The instruction is primarily used for alignment purposes or to provide a target for breakpoints without altering the program state. Since it has no operands, it cannot trigger any floating-point exceptions.