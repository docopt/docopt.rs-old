# GNU Make 3.81
#
SHELL = /bin/sh
RUSTC = rustc

objects = docopt.so test_docopt run_tests agnostic_testee

# TODO: in fututre need to remove it and use whole agnostic test suite
AGNOSTIC_TEST_IDS = `seq 1 3`

docopt: $(objects)

docopt.so: docopt.rs docopt.rc
	$(RUSTC) -L ./rust-pcre docopt.rc

test_docopt: docopt.rs docopt.rc
	$(RUSTC) docopt.rc --test -o test_docopt -L ./ -L ./rust-pcre

run_tests:
	./test_docopt

agnostic_testee: agnostic_testee.rs
	$(RUSTC) agnostic_testee.rs -L ./ -L ./rust-pcre

agnostic_tests:
	python ./python_docopt/language_agnostic_test/language_agnostic_tester.py ./agnostic_testee $(AGNOSTIC_TEST_IDS)

clean:
	rm -f ./*.so ./test_docopt ./agnostic_testee 
