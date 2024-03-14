.DEFAULT_GOAL := all

DAYS := $(wildcard day*)
DAYS_BIN := $(foreach day,$(DAYS),$(day)/$(day))

define GEN_DAY_RULE
$1/$1: $(wildcard $1/*.s) $(wildcard lib/*.s)
	$$(MAKE) -C $1 $1
endef

$(foreach day,$(DAYS),$(eval $(call GEN_DAY_RULE,$(day))))

.PHONY: all
all: $(DAYS_BIN)

.PHONY: clean
clean:
	$(foreach day,$(DAYS),$(MAKE) -C $(day) clean ;) 
	rm -f lib/*.o

