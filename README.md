# KAYOS: Kind Of Stupid Operating System

Just a series of stuff from https://os.phil-opp.com/

## Notable steps

### Install `bootimage`

```shell
cargo install bootimage
```

### Build boot image

```shell
cargo bootimage
```

### Booting in QEMU 

```shell
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kayos/debug/bootimage-kayos.bin
```