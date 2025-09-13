KVERSION := `uname -r`
KDIR ?= /lib/modules/${KVERSION}/build

all:
	$(MAKE) -f $(KDIR)/Makefile O=$(KDIR) M=$(PWD) modules

clean:
	$(MAKE) -f $(KDIR)/Makefile M=$(PWD) clean
