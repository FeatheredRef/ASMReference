> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SMSW

Stores the contents of the SW (Saved Writes) register into the specified destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal Register | reg (r16) |
| Internal Register | m2 |

DO NOT support LOCK

This instruction is only available in compatibility mode. In 64-bit mode, this instruction is not supported and will trigger an invalid opcode exception.

The destination register or memory location SHALL be 16 bits wide. Attempting to use a 32-bit or 64-bit destination operand will result in an invalid operation.