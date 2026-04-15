> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[CAPABILITIES]

GETSEC[CAPABILITIES] retrieves the capabilities of the Secure Arbitration Mode (SAM) and stores them in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r64 |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. Execution in compatibility mode or 32-bit mode SHALL result in an invalid opcode exception.

To avoid unexpected behavior, the programmer MUST verify that the processor supports the GETSEC instruction by checking the CPUID leaves before invocation. Failure to do so on unsupported hardware will result in a #UD exception.