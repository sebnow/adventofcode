input/%.txt:
	$(eval year := $(patsubst input/%/,%,$(dir $@)))
	$(eval day := $(patsubst day%.txt,%,$(notdir $@)))
	$(eval day := $(day:0%=%))
	$(eval url := https://adventofcode.com/${year}/day/${day}/input)
	@echo GET "${url}"
	@curl "${url}" --cookie "session=${SESSION}" -o "$@"
