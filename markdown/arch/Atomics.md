# Atomics

Formalized around the Pentium era, it serves the purpose of helping to maintain expected behaviors on hyper-threading and/or multi-processor environments. "Atomic" in this case means that the referred execution will be one instruction at a time, following issue order. Thus serialized and ordered.

By interacting with a [memory](</arch/memory>) location, the processor will naturally be serialized, but not globally ordered. And if a processor writes into a memory space where another core has in its [L-Cache](</arch/l-cache>), it will be invalidated. By using the "LOCK" prefix, the instruction issues a LOCK signal on the [bus], and the Atomic behavior is therefore guaranteed.

The standard usage of Atomics is in read-and-write instructions, [cmpxchg](</instructions/x86/cmpxchg>) for instance. Used for mutex-free parallel state maintenance. Thus achieving better efficiency.

In conclusion, Atomics guarantee serial and ordered behavior on instructions. Programmers exploit this to build mutex-free implementations.
