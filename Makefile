# make file to hold the logic of build and test setup
ZK_VERSION ?= 3.5.6

# Apache changed the name of the archive in version 3.5.x and seperated out
# src and binary packages
ZK_MINOR_VER=$(word 2, $(subst ., ,$(ZK_VERSION)))
ifeq ($(shell test $(ZK_MINOR_VER) -le 4; echo $$?),0)
  ZK = zookeeper-$(ZK_VERSION)
else
  ZK = apache-zookeeper-$(ZK_VERSION)-bin
endif
ZK_URL = "https://archive.apache.org/dist/zookeeper/zookeeper-$(ZK_VERSION)/$(ZK).tar.gz"


.DEFAULT_GOAL := test

$(ZK):
	curl -o $(ZK).tar.gz $(ZK_URL)
	tar -zxf $(ZK).tar.gz
	rm $(ZK).tar.gz

zookeeper: #$(ZK)
	# we link to a standard directory path so then the tests dont need to find based on version
	# in the test code. this allows backward compatable testing.
	ln -s $(ZK) zookeeper
	mv zookeeper/conf/zoo_sample.cfg zookeeper/conf/zoo.cfg
	zookeeper/bin/zkServer.sh start

.PHONY: setup
setup: zookeeper

.PHONY: lint
lint:
	cargo fmt
	cargo clippy

.PHONY: build
build:
	cargo build

.PHONY: test
test: build zookeeper
	cargo test

.PHONY: clean
clean:
	rm -f apache-zookeeper-*.tar.gz
	rm -f zookeeper-*.tar.gz
	rm -rf apache-zookeeper-*/
	rm -rf zookeeper-*/
	rm -f zookeeper
