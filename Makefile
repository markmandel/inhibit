#
# Copyright 2020 Google LLC All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

.DEFAULT_GOAL := help

## Installs to /usr/local/bin/ and puts icon in /usr/share/icons/
install: deps release
	sudo cp ./target/release/inhibit /usr/local/bin/
	sudo chmod o+x /usr/local/bin/inhibit
	sudo cp ./icons/*.png /usr/share/icons/
	sudo chmod o+r /usr/share/icons/inhibit-*.png

## Builds the release
build: release
	cp -r ./icons ./target/release

## Runs the development version
run:
	cargo run

release:
	cargo build --release

## Installs linux dependencies
deps:
	sudo apt install libappindicator3-dev libgtk-3-dev gcc clang

# Credit: https://gist.github.com/klmr/575726c7e05d8780505a
.PHONY: help
help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=19 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'|more $(shell test $(shell uname) == Darwin && echo '-Xr')