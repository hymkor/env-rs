ifeq ($(OS),Windows_NT)
    SHELL=CMD.EXE
    SET=set
    NUL=nul
    EXE=.exe
    VERSION:=v$(shell powershell "(cargo metadata --no-deps --format-version 1 | ConvertFrom-Json).packages.version")
    RM=del

else
    SET=export
    NUL=/dev/null
    EXE=
    AWK=gawk
    RM=rm
    VERSION:=v$(shell $(AWK) '/version/{ gsub(/[=\"]/,"",$$NF) ; print $$NF ; exit }' Cargo.toml)
endif

PACKAGE:=$(notdir $(CURDIR))
NAME:=$(subst -rs,,$(PACKAGE))

all:
	@echo make dist/manifest/release/clean-dist

clean-dist:
	$(RM) $(NAME)-*.zip

TARGET=$(ARCH)-$(VENDOR)-$(SYS)-$(ABI)
_dist:
	cargo build --release --target $(TARGET)
	zip -j $(NAME)-$(VERSION)-$(SYS)-$(ARCH).zip target/$(TARGET)/release/$(NAME)$(EXE)

dist:
	$(MAKE) _dist ARCH=i686   VENDOR=pc SYS=windows ABI=msvc
	$(MAKE) _dist ARCH=x86_64 VENDOR=pc SYS=windows ABI=msvc

manifest:
	make-scoop-manifest *-windows-*.zip > $(PACKAGE).json

release:
	gh release create -d --notes "" -t $(VERSION) $(VERSION) $(wildcard $(NAME)-$(VERSION)-*.zip)

tag:
	git tag $(VERSION)

version:
	echo pub const VERSION: ^&str = "$(NAME) $(VERSION) for Windows"^;> .\src\version.rs
	cargo fmt

.PHONY: dist manifest release
