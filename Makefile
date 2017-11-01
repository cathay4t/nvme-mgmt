_libnvme_so_dir = "target/debug/"
_libnvme_h_dir = "src/libnvme_for_c/"
TEST_EXEC = test/libnvme_test
CFLAGS += -I$(_libnvme_h_dir)
LDFLAGS += -L$(_libnvme_so_dir) -lnvme

LIBNVME_VERSION=0.1.0
SONAME=$(LIBNVME_VERSION)
DEVLIB = libnvme.so
LIBS = target/debug/$(DEVLIB).$(SONAME)

LIBS:
	cargo build --all
	ln -sf $(DEVLIB) $(LIBS)

all: $(LIBS) $(TEST_EXEC)

check: $(LIBS) $(TEST_EXEC)
	sudo env LD_LIBRARY_PATH=$(_libnvme_so_dir) ./$(TEST_EXEC)

clean:
	rm -rf $(TEST_EXEC)
	rm -rf $(LIBS)
	cargo clean
