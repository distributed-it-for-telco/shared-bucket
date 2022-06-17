INTERFACES = shared-bucket
ACTORS = customers service-vendors shared-bucket-api

all: $(ACTORS) $(INTERFACES)

$(ACTORS):
	$(MAKE) -C actors/$@

$(INTERFACES):
	$(MAKE) -C interfaces/$@

