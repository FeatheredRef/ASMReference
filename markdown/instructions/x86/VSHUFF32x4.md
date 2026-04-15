> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSHUFF32x4

Shuffles four 32-bit double words from two input vectors according to an immediate byte, storing the result in a destination vector.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | xmm reg |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available when the AVX extension is supported. It requires the destination register to be an xmm register.

The immediate byte MUST be provided at encode time; it cannot be a register. Using an invalid immediate value will result in the instruction ignoring the shuffle and potentially maintaining the original destination values or producing undefined results depending on the specific processor implementation. To avoid unexpected behavior, SHALL ensure the immediate byte corresponds to a valid shuffle pattern defined in the Intel SDM.