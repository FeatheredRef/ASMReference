> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FXCH

Exchanges the contents of a specified floating-point register with the contents of the register at the top of the floating-point register stack (ST(0)).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(i) | ST(0) |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It operates exclusively on the x87 floating-point register stack.

The register index $i$ MUST be between 0 and 7. If the specified register is ST(0), the instruction performs no operation. The instruction does not affect the floating-point status word or control word.