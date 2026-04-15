> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BOUND

Checks if the index value in the source operand is within the range specified by the lower and upper bounds stored in the destination operand. If the index is lower than the lower bound or higher than the upper bound, a #NP (processor exception) is generated.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | mN |

DO NOT support LOCK

The BOUND instruction is not supported in 64-bit mode. It is only available when the processor is operating in compatibility mode. In 64-bit mode, attempting to execute this instruction will result in an invalid opcode exception.

Users SHALL ensure the destination memory operand is correctly aligned to avoid alignment check exceptions. Because this instruction is legacy and lacks support in native 64-bit mode, software MUST use conditional jumps or comparison instructions (CMP) to perform range checking in 64-bit environments.