> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LODSW

Loads a word from the memory location pointed to by the source index register (RSI or ESP/RBP in 16-bit mode) into the AX register and increments or decrements the source index register by 2, depending on the direction flag (DF).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m16 | r16 (AX) |

DO NOT support LOCK

This instruction is only available in 16-bit operand size mode. It is supported in 64-bit mode only when the processor is operating in compatibility mode with a 16-bit operand size override.

The behavior of the index register update depends on the DF flag: if DF is 0, the source index is incremented; if DF is 1, the source index is decremented. Users MUST ensure the DF flag is set to the intended direction using `CLD` or `STD` to avoid incorrect memory traversal.