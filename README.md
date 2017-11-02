# nvme-mgmt
WIP: NVMe management interface and tools

## Features
 * Rust create 'nvme-mgmt' with CLI and lib.
 * C library -- libnvme.so and libnvme.h
 * Python binding of libnvme

## Compile and Test

```bash
make
# The 'make check' will run against /dev/nvme0 for now.
make check
```
