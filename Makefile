# Install Make in Windows:
# >>> choco install make

.DEFAULT_GOAL := commands

ifeq ($(OS), Windows_NT)
  COMMIT = @git status --porcelain | findstr /R "^ M ^A ^D ^R ^C ^UU ^??" >nul && \
  (git add . && git commit -m "Update") || @echo No commits to make.
else
  COMMIT = @git status --porcelain | grep -q "^\( M\|M \|A \|D \|R \|C \|UU \|?? \)" && \
  (git add . && git commit -m "Update") || @echo No commits to make.
endif

commands:
	@echo Commands:
	@echo     make commit ---------- Branch commit (using git)
	@echo     make push ------------ Push your project for repository
	@echo     make fmt ------------- Format code
	@echo     make build ----------- Compile code
	@echo     make release --------- Compile code for release
	@echo     make clean ----------- Delete entire build directory
	@echo ----------------------------------------------------------
	@echo     (c) 2025 - Ssec - Makefile commands

fmt:
	@cargo fmt

commit:
	$(COMMIT)

push: fmt commit
	@git push -u origin
	@git push -u hub

build: fmt
	@cargo build

release:
	@cargo build --release

clean:
	@cargo clean
