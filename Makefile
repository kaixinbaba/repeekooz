ZK_VERSION ?= 3.5.6

ZK = apache-zookeeper-$(ZK_VERSION)-bin
ZK_URL = "https://archive.apache.org/dist/zookeeper/zookeeper-$(ZK_VERSION)/$(ZK).tar.gz"


.DEFAULT_GOAL := test

$(ZK):
	curl -o $(ZK).tar.gz $(ZK_URL)
	tar -zxf $(ZK).tar.gz
	rm $(ZK).tar.gz

zookeeper: #$(ZK)
	mv $(ZK)/conf/zoo_sample.cfg $(ZK)/conf/zoo.cfg
	$(ZK)/bin/zkServer.sh start

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
