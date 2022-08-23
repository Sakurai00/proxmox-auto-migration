TARGET = aarch64-unknown-linux-gnu
TARGETDIR = ./target
KEYDIR = ./.ssh/id_rsa

build:
	cross build --target=$(TARGET) --release --target-dir $(TARGETDIR)

cp:
	scp -i $(KEYDIR) $(TARGETDIR)/$(TARGET)/release/auto_migration root@RPI01-pve:~/
	scp -i $(KEYDIR) $(TARGETDIR)/$(TARGET)/release/auto_migration root@RPI02-pve:~/

run1:
	ssh -i $(KEYDIR) root@RPI01-pve ./auto_migration -t RPI02-pve -i 500

run2:
	ssh -i $(KEYDIR) root@RPI02-pve ./auto_migration -t RPI01-pve -i 500
