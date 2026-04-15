> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XSAVE

Saves the current state of extended processor registers to a memory region based on a mask. The instruction selectively saves components of the state (such as XMM, YMM, ZMM, and MXCSR) depending on the value specified in the mask and the enabled features in the XCR0 register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| internal state | mN |
| reg | mN |
| imm | mN |

DO NOT support LOCK

The instruction SHALL only be executed when the processor is in 64-bit mode or compatibility mode. Execution in 32-bit mode is not supported unless the specific processor implementation allows it via a specific extension. The instruction REQUIRES the `XCR0` register to be configured to enable the specific state components being saved; failure to do so MAY result in a general-protection exception (#GP).

The destination memory region MUST be aligned on a 64-byte boundary unless the `Reps` prefix is used or the specific `XSAVE` variant (like `XSAVEC` or `XSAVES`) is employed. Failure to maintain 64-byte alignment SHALL result in a general-protection exception (#GP). Additionally, if the memory region overlaps with the state being saved, the behavior is undefined.