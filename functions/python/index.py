import os
import importlib.util
import inspect

# Get the directory this file is in
dir_path = os.path.dirname(os.path.realpath(__file__))
# Append /functions
dir_path = os.path.join(dir_path, "functions")

# Dictionary to store the functions
function_dict = {}

# Iterate through all files in the directory
for filename in os.listdir(dir_path):
    if filename.endswith(".py"):
        # Get the full path of the file
        file_path = os.path.join(dir_path, filename)
        
        # Get the module name (filename without .py extension)
        module_name = os.path.splitext(filename)[0]
        
        # Create a module specification
        spec = importlib.util.spec_from_file_location(module_name, file_path)
        
        # Create a new module based on the spec
        module = importlib.util.module_from_spec(spec)
        
        # Execute the module
        spec.loader.exec_module(module)
        
        # Add only functions from the module to the dictionary
        for item_name in dir(module):
            item = getattr(module, item_name)
            if inspect.isfunction(item) and not item_name.startswith("__"):
                function_dict[item_name] = item

# Now function_dict contains only the functions from .py files in the directory
print(function_dict)