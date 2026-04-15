> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STC

Sets the Carry Flag (CF) to 1.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | CF |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. It does not require any specific architectural constraints regarding the operand size or address size.

Since STC modifies the EFLAGS register, it is critical to ensure that any subsequent conditional jumps or set-instructions (e.g., JC, JNC) that rely on the Carry Flag are correctly synchronized with this operation. This instruction affects only the CF bit; all other flags remain unchanged.