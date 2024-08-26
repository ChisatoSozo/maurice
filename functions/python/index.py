import os
import importlib.util
import inspect
import sys
import json
import threading

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

def handle_generator(gen, caller_id):
    try:
        for item in gen:
            result = {
                "caller_id": caller_id,
                "result": item,
                "done": False
            }
            print(json.dumps(result))
            sys.stdout.flush()
    except Exception as e:
        error_result = {
            "caller_id": caller_id,
            "error": str(e),
            "done": True
        }
        print(json.dumps(error_result))
        sys.stdout.flush()
    else:
        final_result = {
            "caller_id": caller_id,
            "result": None,
            "done": True
        }
        print(json.dumps(final_result))
        sys.stdout.flush()

def handle_result(result, caller_id):
    if inspect.isgenerator(result):
        thread = threading.Thread(target=handle_generator, args=(result, caller_id))
        thread.start()
    else:
        # For non-generator results, send the result immediately
        output = {
            "caller_id": caller_id,
            "result": result,
            "done": True
        }
        print(json.dumps(output), flush=True)
        sys.stdout.flush()

if __name__ == "__main__":
    print("Ready", flush=True)
    while True:
        # Read line from stdin
        print("Waiting for input...", flush=True)
        line = sys.stdin.readline().strip()

        # If the line is empty, break the loop
        if not line:
            break

        print("Received:", line, flush=True)
        
        # The line is a JSON object with a function name and arguments
        # Parse the JSON object
        json_obj = json.loads(line)

        # Get the function name, arguments, and caller_id
        func_name = json_obj["name"]
        args = json_obj["args"]
        caller_id = json_obj["caller_id"]

        # Get the function from the dictionary
        func = function_dict[func_name]

        try:
            #call func(**args) in a separate thread and feed the result to handle_result
            if args:
                handle_result(func(**args), caller_id)
            else:
                handle_result(func(), caller_id)
            
        except Exception as e:
            # Handle any exceptions and send an error message
            error_output = {
                "caller_id": caller_id,
                "error": str(e),
                "done": True
            }
            print(json.dumps(error_output), flush=True)
            sys.stdout.flush()