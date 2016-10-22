# Get protc from: https://github.com/google/protobuf/releases
# sudo apt-get install protobuf-compiler
~/src/bin/protoc --proto_path ./src/tests/ --plugin ~/.cargo/bin/protoc-gen-rust --rust_out ./src/tests src/tests/test_proto.proto


