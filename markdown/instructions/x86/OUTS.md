> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# OUTS

Outputs a string of data from memory to the I/O port specified by the DX register. The number of bytes transferred is determined by the value in the CX or ECX register. After each transfer, the memory address pointer is incremented or decremented based on the DF (Direction Flag) in RFLAGS.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m8, m16, m32 | dx |

DO NOT support LOCK

This instruction is ONLY available in 32-bit mode or compatibility mode. In 64-bit mode, the instruction is not supported and will trigger an invalid opcode exception.

The instruction MUST have the DF flag configured correctly to avoid reading from incorrect memory addresses; if DF=0, the pointer is incremented, and if DF=1, the pointer is decremented. The use of the `rep` prefix is REQUIRED to transfer multiple units of data; otherwise, only a single element is transferred and ECX is not decremented. Failure to ensure ECX is non-zero before execution with a `rep` prefix may lead to an unexpected number of iterations based on the wrapped value of the register.