rust-docopt
===========

This is a port of Python docopt library http://github.com/docopt/docopt

** WORK IS CURRENTLY IN PROGRESS **


Requirements
-------------

- Rust == 0.4  // rust-pcre currently supports this latest version
- GNU Make == 3.81  // you can try other versions as well


Building & tests
----------------
Run following commands to init and update git submodules::

   git submodule init
   git submodule update

To build and run language agnostic tests::

    make

Run only agnostic tests::

    make run_agnostic_tests

Clean::

    make clean

By default only first test is used, as by now nothing is
implemented yet, i.e. all tests are red.
    
** WORK IS CURRENTLY IN PROGRESS **
