SOURCE_DIR = $(PWD)
# `notdir` returns the part after the last `/`
# so if the source was "/some/nested/project", only "project" remains
BUILD_DIR  = ~/tmp/$(notdir $(SOURCE_DIR))

build: wsl.sync
	cd $(BUILD_DIR) && cargo build

run: wsl.sync
	cd $(BUILD_DIR) && cargo run

check: wsl.sync
	cd $(BUILD_DIR) && cargo check --message-format=json

wsl.sync:
	mkdir -p $(BUILD_DIR)
	rsync -av $(SOURCE_DIR)/ $(BUILD_DIR)/ --exclude .git --exclude target
