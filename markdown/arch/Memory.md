# Memory

> Data in ram
> Note: This text will cover memory assuming the OS is linux

Obviously, in computing we use memory. It being used to ensure, enforce, keep, and/or manage states, memory is what RAM keeps. It is usually partitioned as heap, and [stack](</arch/stack>).

The memory is segmented in N pages of four kibibytes (4096 bytes), when an instance allocate memory, `(N+4095)/4096` (N=bytes allocated) pages have the "ownership" set to the process.

A memory allocation in linux have protection flags, and flags. The protection flags define if pages can be written, readen, or executed. As for flags, define the mapping. A table with both flags, and protection flags is bellow.

| Protection Flag | Description            |
|-----------------|------------------------|
| PROT_NONE       | No access permitted    |
| PROT_READ       | Pages can be read      |
| PROT_WRITE      | Pages can be written   |
| PROT_EXEC       | Pages can be executed  |

| Flag                  | Description                                                                   |
|-----------------------|-------------------------------------------------------------------------------|
| MAP_SHARED            | Writes visible to others; written to file                                     |
| MAP_PRIVATE           | Copy-on-write; writes not visible to others                                   |
| MAP_ANONYMOUS         | Not backed by a file; fd should be -1                                         |
| MAP_FIXED             | Place mapping exactly at addr (dangerous; can replace existing mappings)      |
| MAP_FIXED_NOREPLACE   | Like MAP_FIXED but fail if address is occupied                                |
| MAP_GROWSDOWN         | Stack-like mapping; grows downward                                            |
| MAP_HUGETLB           | Use huge pages                                                                |
| MAP_LOCKED            | Lock pages in memory as with `mlock()`                                        |
| MAP_NORESERVE         | Do not reserve swap space                                                     |
| MAP_POPULATE          | Prefault pages (read-ahead)                                                   |
| MAP_STACK             | Hint that this is for a stack                                                 |
| MAP_SYNC              | Shared file mapping with synchronous write-back (DAX; since Linux 4.15)       |
