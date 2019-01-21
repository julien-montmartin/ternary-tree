#!/bin/bash


echo "Download and build kcov"

wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
cd kcov-master
mkdir build
cd build
cmake ..
make
make install DESTDIR=../../kcov-build
cd ../..
rm -rf kcov-master

set -x

KCOV=./kcov-build/usr/local/bin/kcov
TESTS=$(find ./target/debug -maxdepth 1 -executable -iname "tests-*")

for t in ${TESTS}; do

	DIR=target/cov/$(basename ${t})

	mkdir -p ${DIR}

	echo "Run kcov on ${t} in ${DIR}"

	${KCOV} --exclude-pattern=/.cargo,/usr/lib --verify ${DIR} ${t}

done

bash <(curl -s https://codecov.io/bash)

echo "Uploaded code coverage"
