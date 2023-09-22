# Dependencies

make, gcc (assembler), qemu, ld, cargo/rust

# Build

```
make 
```

# Run

To boot the kernel with QEMU

```
make run
```

# Debug

Commands to execute from the root of the repo.

In one terminal, execute qemu with a gdb listen server.
```
make run_debug
```

In another terminal, execute a script that will connect the gdb client to qemu.
The script is customizable in scripts/debuug.gdb.
```
bash ./scripts/debug.sh
```



