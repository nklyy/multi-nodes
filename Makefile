.SILENT: test build run

CYAN=\033[0;36m
RESET=\033[0m

pprint = echo -e "${CYAN}::>${RESET} ${1}"
completed = $(call pprint,Completed!)

test:
	$(call pprint, Runnning tests...)
	cargo test
	$(call completed)

build: clean deps
	$(call pprint, Building app...)
	cargo build
	$(call completed)

run:
	$(call pprint, Running app...)
	cargo run
	$(call completed)