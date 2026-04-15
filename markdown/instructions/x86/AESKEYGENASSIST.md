> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AESKEYGENASSIST

AESKEYGENASSIST performs the key expansion assistance operation for the AES algorithm. It takes an input value, applies a specific transformation involving a rotation, a substitution via the S-box, and a XOR operation with a round constant, then stores the resulting value in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available if the processor supports the AES-NI instruction set. It operates exclusively on XMM registers; it is NOT supported for memory operands or general-purpose registers.

The instruction requires the processor to be in 64-bit mode or compatibility mode. Users MUST ensure that the AES-NI feature flag is enabled in the CPUID to avoid an `#UD` (Undefined Instruction) exception. Since this instruction modifies the destination register, the original input value is lost unless previously backed up to another register or memory.