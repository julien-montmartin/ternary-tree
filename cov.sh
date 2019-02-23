#!/bin/bash

cat <<EOF

************************************************************************
*
* Donwload latest Kcov AppImage
*
************************************************************************

EOF

REPO=julien-montmartin/kcov-appimage
HTML=$(wget -q -O - https://github.com/${REPO}/releases/latest)
RELEASE=$(grep -o -E /${REPO}/releases/download/[^/]+/kcov-[^.]+\\.AppImage <<< ${HTML})
URL=https://github.com/${RELEASE}
KCOV=./kcov.AppImage

wget -q -O ${KCOV} ${URL}
chmod +x ${KCOV}

CMD="${KCOV} --version"
VER=$(eval ${CMD})

echo "Running ${CMD} says ${VER}"

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

	${KCOV} --exclude-pattern=/.cargo,/usr/lib --verify ${DIR} ${T}

done

cat <<EOF

************************************************************************
*
* Upload coverage results
*
************************************************************************

EOF

bash <(curl -s https://codecov.io/bash)
