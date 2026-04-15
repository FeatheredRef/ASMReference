> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRFSBASE

WRFSBASE writes a value from a register to the FS segment base address.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | FS base |
| imm | #I |
| mN | #I |

DO NOT support LOCK

The instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

To avoid an invalid opcode exception, ensure the processor supports the `FSGSBASE` CPUID feature flag. If the `FSGSBASE` feature is disabled, the instruction shall be treated as an undefined opcode.