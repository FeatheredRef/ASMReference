> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XSUSLDTRK

XSUSLDTRK suspends the loading of the state for the specified XCR0-enabled components, preventing them from being loaded during a subsequent XRSTOR operation.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the processor to support the XSAVE feature set. If the instruction is executed on a processor that does not support the specified state components or if the operation is invalid for the current processor configuration, it MAY trigger a general-protection exception (#GP).

To avoid unintended state loss, ensure that the immediate operand correctly masks only the components intended for suspension. Because this instruction affects the behavior of future XRSTOR executions, failing to properly manage the suspension state can lead to the corruption of SIMD or extended state registers when restoring context.