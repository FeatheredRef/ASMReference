> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KADDB

This instruction is not a part of the official x86-64 Instruction Set Architecture (ISA) as defined by Intel or AMD. No such instruction mnemonic exists in the standard x86-64 opcode map.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is not defined in any architectural mode (Real Mode, Protected Mode, or Long Mode). Consequently, attempting to execute an opcode that does not map to a known instruction will result in an Invalid Opcode exception (#UD).

Since this instruction does not exist in the x86-64 ISA, any attempt to use it in assembly source code will result in a compilation error. To avoid undefined behavior or crashes, only use mnemonics documented in the Intel 64 and IA-32 Architectures Software Developer's Manual.