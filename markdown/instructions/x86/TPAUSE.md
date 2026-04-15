> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TPAUSE

TPAUSE pauses the execution of the processor for a specified number of clock cycles. If the time limit is reached or a specified event occurs, the processor resumes execution.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |

DO NOT support LOCK

TPAUSE is only available in 64-bit mode. It is NOT supported in compatibility mode. The instruction requires the `TPAUSE` feature flag to be supported by the processor (CPUID.07H.EBX[0]).

The time delay is specified by the value in the source register. If the value is 0, the instruction behaves as a `NOP`. The actual pause duration is implementation-dependent and may be slightly longer than the requested number of cycles. To avoid unexpected system hangs, the value provided MUST be within the range supported by the specific processor implementation. If the processor does not support the requested delay, it MAY truncate the value.