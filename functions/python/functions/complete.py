import sys, os

current_dir = os.path.dirname(os.path.abspath(__file__))
parent_dir = os.path.dirname(current_dir)
sys.path.append(parent_dir)

class CompletionModel:
    def __init__(self, model_name: str, cache_quant: int):
        import torch
        gpu_list = [torch.cuda.get_device_name(i) for i in range(torch.cuda.device_count())]
        print("Available GPUs:", gpu_list)

        found_rtx_4090 = False

        # Try to find and select the RTX 4090
        for i, gpu in enumerate(gpu_list):
            if "4090" in gpu:
                torch.cuda.set_device(i)
                print(f"Selected GPU: {gpu}")
                found_rtx_4090 = True
                break
        # If 4090 is not found, print a message
        if not found_rtx_4090:
            print("RTX 4090 not found. Using the default GPU.")
        from exllamav2 import ExLlamaV2, ExLlamaV2Config, ExLlamaV2Cache, ExLlamaV2Cache_Q4, ExLlamaV2Cache_Q6, ExLlamaV2Cache_Q8, ExLlamaV2Tokenizer
        from exllamav2.generator import ExLlamaV2DynamicGenerator
        self.name = model_name
        model_dir = os.path.join(os.path.dirname(__file__), "../models", model_name)
        config = ExLlamaV2Config(model_dir)
        config.arch_compat_overrides()
        self.model = ExLlamaV2(config)

        if cache_quant == 4:
            self.cache = ExLlamaV2Cache_Q4(self.model, max_seq_len = 8192, lazy = True)
        elif cache_quant == 6:
            self.cache = ExLlamaV2Cache_Q6(self.model, max_seq_len = 8192, lazy = True)
        elif cache_quant == 8:
            self.cache = ExLlamaV2Cache_Q8(self.model, max_seq_len = 8192, lazy = True)
        else:
            self.cache = ExLlamaV2Cache(self.model, max_seq_len = 8192, lazy = True)
        self.model.load_autosplit(self.cache, progress = True)

        print("Loading tokenizer...")
        self.tokenizer = ExLlamaV2Tokenizer(config)

        self.generator = ExLlamaV2DynamicGenerator(
            model = self.model,
            cache = self.cache,
            tokenizer = self.tokenizer,
        )

        self.generator.warmup()
    
    def complete(self, prompt: str, max_new_tokens: int) -> str:
        return self.generator.generate(prompt = prompt, max_new_tokens = max_new_tokens, add_bos = True)

loaded_model: CompletionModel = None

def list_completion_models(blank_arg: bool) -> list[str]:
    #list all folders in the models directory next to this script
    models = os.listdir(os.path.join(os.path.dirname(__file__), "../models"))
    #filter out any files that are not directories
    models = [model for model in models if os.path.isdir(os.path.join(os.path.dirname(__file__), "../models", model))]
    #filter out any models that contain "raw" in the name
    models = [model for model in models if "raw" in model]
    return models

def load_completion_model(model_name: str, cache_quant: int) -> bool:
    global loaded_model
    if loaded_model is not None:
        del loaded_model
        loaded_model = None
    loaded_model = CompletionModel(model_name, cache_quant)
    print(f"Loaded model: {model_name}")
    print(loaded_model)
    return True

def get_loaded_completion_model_name(blank_arg: bool) -> str:
    global loaded_model
    if loaded_model is None:
        return ""
    return loaded_model.name

def complete(prompt: str, max_new_tokens: int) -> str:
    global loaded_model
    if loaded_model is None:
        return "No model is loaded"
    return loaded_model.complete(prompt, max_new_tokens)

