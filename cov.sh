#!/bin/bash

cat <<EOF

************************************************************************
*
* Donwload latest Kcov AppImage
*
************************************************************************

EOF

TRAMPOLINE=https://raw.githubusercontent.com/julien-montmartin/kcov-appimage/master/trampoline.txt
curl -s ${TRAMPOLINE} | curl -sLK -

chmod +x ./kcov-x86_64.AppImage
echo "Running AppImage of $(./kcov-x86_64.AppImage --version)"

cat <<EOF

************************************************************************
*
* Run Kcov on tests
*
************************************************************************

EOF

TESTS=$(find ./target/debug -maxdepth 1 -executable -iname "tests-*")

for T in ${TESTS}; do

	DIR=target/cov/$(basename ${T})

	mkdir -p ${DIR}

	echo "Run kcov on ${t} in ${DIR}"

	./kcov-x86_64.AppImage --exclude-pattern=/.cargo,/usr/lib --verify ${DIR} ${T}

done

cat <<EOF

************************************************************************
*
* Upload coverage results
*
************************************************************************

EOF

bash <(curl -s https://codecov.io/bash)
