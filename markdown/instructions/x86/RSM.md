> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RSM

Resumes operation from a Save State area, restoring the processor state from the memory location pointed to by the RSM state area and returning the processor to the previous execution environment.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| reg | #I |
| imm | #I |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode. It is used exclusively in conjunction with the `SENTER` and `GETSEC` instructions within the Trusted Execution Technology (TXT) framework. If the processor is not in a state where resuming from a save area is valid, or if the `SMM` (System Management Mode) is not configured to support this transition, the instruction MAY trigger a general protection fault (#GP).

To avoid unexpected system resets or #GP faults, the software MUST ensure that the Save State area is properly aligned and that the `GETSEC[RSM]` leaf has been previously validated. Failure to provide a valid state area structure as defined by the Intel TXT specification WILL result in an invalid state transition.