> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[PARAMETERS]

GETSEC retrieves security-related parameters from the processor and stores them in the specified destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | reg |

DO NOT support LOCK

The instruction is only available in 64-bit mode. If executed in compatibility mode or 32-bit mode, it SHALL trigger an invalid opcode exception.

The specific parameter retrieved depends on the immediate value provided; providing an unsupported immediate value SHALL result in the destination register being undefined or the instruction triggering a general protection fault (#GP) depending on the processor implementation. Ensure that the destination register is a 64-bit general-purpose register to avoid data truncation.