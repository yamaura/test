#!/bin/bash
BASE_URL=http://ftp.iij.ad.jp/pub/linux
CENTOS_RELEASE=(
centos-vault/7.0.1406
centos-vault/7.1.1503
centos-vault/7.2.1511
centos-vault/7.3.1611
centos-vault/7.4.1708
centos-vault/7.5.1804
centos-vault/7.6.1810
centos-vault/7.7.1908
)
REPO=(
os/x86_64/Packages
updates/x86_64/Packages
)
package=kernel-devel-$(uname -r)

for centos_release in ${CENTOS_RELEASE[@]}; do
	for repo in ${REPO[@]}; do
		url=${BASE_URL}/${centos_release}/${repo}/${package}.rpm
		if curl -sfLo/dev/null ${url}; then
			yum install -y ${url}
		fi
	done
done
