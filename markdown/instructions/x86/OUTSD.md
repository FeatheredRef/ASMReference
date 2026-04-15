> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# OUTSD

Reads a dword from the memory location pointed to by `ESI` (or `RSI`) and outputs it to the I/O port specified by the `DX` register. After the operation, the index register is incremented or decremented by 4, depending on the direction flag (DF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m4 | reg (DX) |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode. It is only available in 32-bit mode and compatibility mode.

The instruction relies on the Direction Flag (DF) in the `RFLAGS` register to determine if `ESI`/`RSI` is incremented (DF=0) or decremented (DF=1). Failure to set the DF correctly will result in the wrong memory address being accessed in subsequent iterations of a string operation.