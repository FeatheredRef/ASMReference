> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XSAVEC

XSAVEC saves the current state of the processor's registers to a memory location. It uses a compacted format by skipping the saving of state components that are in their initial state (as defined by the XCR0 register) or components that have not been modified since the last initialization.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | mN |

DO NOT support LOCK

XSAVEC is only available when the processor is operating in 64-bit mode or compatibility mode. The instruction requires the `XSAVE` feature to be enabled in the CPUID and the corresponding state components to be enabled in the `XCR0` register.

The destination memory region mN MUST be aligned on a 64-byte boundary; failure to do so SHALL result in a general-protection exception (#GP). The memory region MUST be large enough to hold the state components enabled in `XCR0`, otherwise a general-protection exception (#GP) SHALL occur. Users MUST ensure that the `XCR0` register is correctly configured before execution to avoid saving unnecessary state or triggering exceptions.