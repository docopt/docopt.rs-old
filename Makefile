# GNU Make 3.81
#
SHELL = /bin/sh
RUSTC = rustc

objects = docopt.so agnostic_testee run_agnostic_tests

# TODO: in fututre need to remove it and use whole agnostic test suite
AGNOSTIC_TEST_IDS = `seq 1 3`

docopt: $(objects)

docopt.so: docopt.rs docopt.rc
	$(RUSTC) docopt.rc

agnostic_testee:
	$(RUSTC) agnostic_testee.rs -L .

run_agnostic_tests:
		python ./python_docopt/language_agnostic_test/language_agnostic_tester.py ./agnostic_testee $(AGNOSTIC_TEST_IDS)

clean:
	rm *.so agnostic_testee
