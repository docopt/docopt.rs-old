# GNU Make 3.81
#
SHELL = /bin/sh
RUSTC = rustc

objects = docopt.so agnostic_tests

# TODO: in fututre need to remove it and use whole agnostic test suite
AGNOSIC_TEST_ID = 1

docopt: $(objects)
	python ./python_docopt/language_agnostic_test/language_agnostic_tester.py \
	    ./agnostic_testee $(AGNOSIC_TEST_ID)

docopt.so: docopt.rs docopt.rc
	$(RUSTC) docopt.rc

agnostic_tests:
	$(RUSTC) agnostic_testee.rs -L .
