> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AAA

Adjusts the value of the accumulator after an ASCII addition operation. It checks if the lowest 4 bits of the `AL` register are greater than 9 or if the Auxiliary Carry Flag (AF) is set; if either condition is true, it increments `AH` by 1, adds 6 to `AL`, and sets both the AF and CF flags.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | #I |
| #I | mN |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode. It is ONLY available in compatibility mode when executing 16-bit or 32-bit code.

The instruction exclusively operates on the `AL` and `AH` registers. Attempting to use it in a context where 64-bit registers are expected will result in an invalid operation or is simply not supported by the hardware. Users MUST ensure the code is running in a compatible mode to avoid illegal instruction exceptions.