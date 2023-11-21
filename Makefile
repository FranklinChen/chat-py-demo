# Assume datamodel-codegen from pydantic is installed.
# Assume Cargo is installed.

GENERATED_PYDANTIC_CODE = python/chat_py_demo/chat_ast.py
GENERATED_JSON_SCHEMA = chat_ast.json

all:	$(GENERATED_PYDANTIC_CODE)

$(GENERATED_PYDANTIC_CODE):	$(GENERATED_JSON_SCHEMA)
	datamodel-codegen --output-model-type pydantic_v2.BaseModel --input-file-type jsonschema --input $^ --output $@ --use-schema-description --use-annotated --use-generic-container-types --use-standard-collections --use-subclass-enum --use-union-operator --use-unique-items-as-set --enum-field-as-literal all --strip-default-none --disable-timestamp --target-python-version 3.11 --use-field-description

$(GENERATED_JSON_SCHEMA):	src/ast.rs
	cargo run > $@

test:
	cargo test
	maturin develop
	pytest

clean:
	rm -f $(GENERATED_PYDANTIC_CODE) $(GENERATED_JSON_SCHEMA)

.PHONY:	all test clean
