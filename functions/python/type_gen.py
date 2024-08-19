import collections
import os
import importlib.util
import inspect
import json
from typing import get_origin, get_args, Generator, List, Dict, Union, Optional
from dataclasses import is_dataclass, fields

def is_generator(py_type):
    return get_origin(py_type) == Generator or inspect.isgeneratorfunction(py_type) or get_origin(py_type) == collections.abc.Generator

def python_type_to_json_schema(py_type):
    if py_type == str:
        return {"type": "string"}
    elif py_type == int:
        return {"type": "integer"}
    elif py_type == float:
        return {"type": "number"}
    elif py_type == bool:
        return {"type": "boolean"}
    elif py_type == list or get_origin(py_type) == list:
        if get_args(py_type):
            item_type = get_args(py_type)[0]
            return {
                "type": "array",
                "items": python_type_to_json_schema(item_type)
            }
        else:
            return {"type": "array"}
    elif py_type == dict or get_origin(py_type) == dict:
        if get_args(py_type):
            key_type, value_type = get_args(py_type)
            if key_type == str:
                return {
                    "type": "object",
                    "additionalProperties": python_type_to_json_schema(value_type)
                }
        return {"type": "object"}
    elif is_dataclass(py_type):
        return dataclass_to_json_schema(py_type)
    elif get_origin(py_type) == Union:
        return {"anyOf": [python_type_to_json_schema(t) for t in get_args(py_type)]}
    elif get_origin(py_type) == Optional:
        return {"anyOf": [python_type_to_json_schema(get_args(py_type)[0]), {"type": "null"}]}
    elif is_generator(py_type):
        return python_type_to_json_schema(get_args(py_type)[0])
    else:
        print("Unknown type:", py_type, get_origin(py_type), get_args(py_type))
        return {"type": "object"}

def dataclass_to_json_schema(dataclass_type):
    properties = {}
    required = []
    for field in fields(dataclass_type):
        field_schema = python_type_to_json_schema(field.type)
        properties[field.name] = field_schema
        if field.default == field.default_factory:
            required.append(field.name)
    
    return {
        "type": "object",
        "properties": properties,
        "required": required
    }

def generate_function_schema(func):
    signature = inspect.signature(func)
    return_annotation = signature.return_annotation

    properties = {}
    required = []

    for param_name, param in signature.parameters.items():
        properties[param_name] = python_type_to_json_schema(param.annotation)
        if param.default == param.empty:
            required.append(param_name)

    return_schema = python_type_to_json_schema(return_annotation)
    return_schema["stream"] = is_generator(return_annotation)

    schema = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "name": {"type": "string", "const": func.__name__},
            "args": {
                "type": "object",
                "properties": properties,
                "required": required
            },
            "return": return_schema
        },
        "required": ["name", "args", "return"]
    }

    return schema

def load_functions_from_directory(directory):
    function_dict = {}
    for filename in os.listdir(directory):
        if filename.endswith(".py"):
            file_path = os.path.join(directory, filename)
            module_name = os.path.splitext(filename)[0]
            spec = importlib.util.spec_from_file_location(module_name, file_path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            
            for item_name in dir(module):
                item = getattr(module, item_name)
                if inspect.isfunction(item) and not item_name.startswith("__"):
                    function_dict[item_name] = item
    return function_dict

def generate_schemas_for_directory(directory):
    functions = load_functions_from_directory(directory)
    schemas = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {},
        "additionalProperties": False
    }
    for func_name, func in functions.items():
        try:
            schemas["properties"][func_name] = generate_function_schema(func)
        except Exception as e:
            print(f"Error generating schema for {func_name}: {str(e)}")
    return schemas

# Usage
if __name__ == "__main__":
    current_dir = os.path.dirname(os.path.realpath(__file__))
    functions_dir = os.path.join(current_dir, "functions")

    all_schemas = generate_schemas_for_directory(functions_dir)

    print(json.dumps(all_schemas, indent=2))

    with open("functions/function_schemas.json", "w") as f:
        json.dump(all_schemas, f, indent=2)
    print("Schemas have been saved to function_schemas.json")