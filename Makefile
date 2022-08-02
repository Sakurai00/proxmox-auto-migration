TARGET = aarch64-unknown-linux-gnu
TARGETDIR = ./target

build:
	cross build --target=$(TARGET) --release --target-dir $(TARGETDIR)

cp:
	scp $(TARGETDIR)/$(TARGET)/release/auto_migration root@RPI01-pve:~/
	scp $(TARGETDIR)/$(TARGET)/release/auto_migration root@RPI02-pve:~/
