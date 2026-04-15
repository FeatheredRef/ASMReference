> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPISTRI

Compares two 128-bit operands for packed strings of characters, using a specific immediate value to determine the comparison type, and sets the FLAGS register (CF, ZF) based on the result.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |
| imm | #I |

DO NOT support LOCK

The instruction is only available in 64-bit mode or compatibility mode. It requires the SSE4.2 instruction set extension to be supported by the processor.

Ensure that the immediate value used is one of the valid constants defined by the ISA; providing an unsupported immediate value will result in undefined behavior. Since the instruction modifies the CF and ZF flags, it SHALL be noted that it may interfere with conditional branches if flags are not managed correctly. Use of this instruction on non-aligned memory operands may result in performance degradation.