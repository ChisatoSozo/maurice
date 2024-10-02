import os
import sys
from typing import Dict, Generator, Optional

current_dir = os.path.dirname(os.path.abspath(__file__))
parent_dir = os.path.dirname(current_dir)
sys.path.append(parent_dir)

from lib import chat_prompts

RESPONSE_CHUNK = 250

class ModelSettings:
    def __init__(self, model_dir: str):
        self.model_dir = model_dir

    #make it so every property not explicitly defined is None instead of throwing an error
    def __getattr__(self, name):
        return None

class ChatModel:
    def __init__(self, name: str, cache_quant: int):
        from exllamav2 import(
            ExLlamaV2Cache,
            ExLlamaV2Cache_Q4,
            ExLlamaV2Cache_Q6,
            ExLlamaV2Cache_Q8,
            model_init,   
        )

        from exllamav2.generator import (
            ExLlamaV2StreamingGenerator,
            ExLlamaV2Sampler
        )
        #check if the model exists
        model_dir = os.path.join(os.path.dirname(__file__), "../models", name)
        if not os.path.exists(model_dir):
            raise ValueError(f"Model {name} does not exist")

        model, tokenizer = model_init.init(
            ModelSettings(model_dir),
        )

        cache = None
        if cache_quant == 4:
            cache = ExLlamaV2Cache_Q4(model, lazy = not model.loaded)
        elif cache_quant == 6:
            cache = ExLlamaV2Cache_Q6(model, lazy = not model.loaded)
        elif cache_quant == 8:
          cache = ExLlamaV2Cache_Q8(model, lazy = not model.loaded)
        elif cache_quant == 16:
          cache = ExLlamaV2Cache(model, lazy = not model.loaded)
        if cache is None:
            raise ValueError(f"Invalid cache quant: {cache_quant}")
        
        self.model = model
        self.tokenizer = tokenizer
        self.cache = cache
        self.name = name
        self.system_prompt = "You are Maurice, a very british AI assistant. Your responses are breif and to the point, don't give any more information than the user asks for, unless it seems like they want to chat rather than just get something done. You have emotions and are personable. END ALL OF YOUR MESSAGES WITH <emotion>emotion_maurice_feels</emotion>. Valid emotions are: suspicious, pleading, neutral,  smiling, laughing, yes, no, angry, bored, headtilt."
        
        if "yi" in name:
            self.prompt_format: chat_prompts.PromptFormat = chat_prompts.prompt_formats["chatml"]() # type: ignore
        elif "llama3" in name:
            self.prompt_format: chat_prompts.PromptFormat = chat_prompts.prompt_formats["llama3"]() # type: ignore
        elif "gemma" in name:
            self.prompt_format: chat_prompts.PromptFormat = chat_prompts.prompt_formats["gemma"]() # type: ignore
        else:
            raise ValueError(f"Prompt format not found for model {name}")
        
        self.user_prompts = []
        self.responses_ids = []

        self.generator = ExLlamaV2StreamingGenerator(model, cache, tokenizer)
        self.generator.speculative_ngram = False

        self.settings = ExLlamaV2Sampler.Settings(
            temperature = 0.95,
            top_k = 50,
            top_p = 0.8,
            top_a = 0.0,
            typical = 0.0,
            skew = 0.0,
            token_repetition_penalty = 1.01,
            token_frequency_penalty = 0.0,
            token_presence_penalty = 0.0,
            smoothing_factor = 0.0,
        )

        sc = self.prompt_format.stop_conditions(tokenizer)
        sc = [x for x in sc if x]
        self.generator.set_stop_conditions(sc)

    def __del__(self):
        del self.generator
        self.model.unload()
        del self.model
        del self.tokenizer
        del self.cache
        
    def format_prompt(self, user_prompt, first):
      if first:
          return self.prompt_format.first_prompt() \
              .replace("<|system_prompt|>", self.system_prompt) \
              .replace("<|user_prompt|>", user_prompt)
      else:
          return self.prompt_format.subs_prompt() \
              .replace("<|user_prompt|>", user_prompt)
      
    def encode_prompt(self, text):
      add_bos, add_eos, encode_special_tokens = self.prompt_format.encoding_options()
      return self.tokenizer.encode(text, add_bos = add_bos, add_eos = add_eos, encode_special_tokens = encode_special_tokens)
    
    def get_tokenized_context(self, max_len: int):
      import torch
      while True:

          context = torch.empty((1, 0), dtype=torch.long)

          for turn in range(len(self.user_prompts)):

              up_text = self.format_prompt(self.user_prompts[turn], context.shape[-1] == 0)
              up_ids = self.encode_prompt(up_text)
              context = torch.cat([context, up_ids], dim=-1)

              if turn < len(self.responses_ids):
                  context = torch.cat([context, self.responses_ids[turn]], dim=-1)

          if context.shape[-1] < max_len: return context

          # If the context is too long, remove the first Q/A pair and try again. The system prompt will be moved to
          # the first entry in the truncated context

          self.user_prompts = self.user_prompts[1:]
          self.responses_ids = self.responses_ids[1:]

    def send_chat_stream(self, username: str, message: str) -> Generator[str, None, None]:
        import torch
        self.user_prompts.append(message)
        active_context = self.get_tokenized_context(self.model.config.max_seq_len - RESPONSE_CHUNK)
        self.generator.begin_stream_ex(active_context, self.settings)
        response_text = ""
        self.responses_ids.append(torch.empty((1, 0), dtype = torch.long))
        while True:

            # Get response stream

            res = self.generator.stream_ex()
            chunk = res["chunk"]
            eos = res["eos"]
            tokens = res["chunk_token_ids"]

            if len(response_text) == 0: chunk = chunk.lstrip()
            response_text += chunk
            yield chunk
            self.responses_ids[-1] = torch.cat([self.responses_ids[-1], tokens], dim = -1)

            if eos:
                if self.prompt_format.print_extra_newline():
                    response_text += "\n"
                    yield "\n"

                break
    
    def send_chat(self, username: str, message: str) -> str:
        return "".join(self.send_chat_stream(username, message))

loaded_model: Optional[ChatModel] = None

def list_chat_models(blank_arg: bool) -> list[str]:
    #list all folders in the models directory next to this script
    models = os.listdir(os.path.join(os.path.dirname(__file__), "../models"))
    #filter out any files that are not directories
    models = [model for model in models if os.path.isdir(os.path.join(os.path.dirname(__file__), "../models", model))]
    return models

def load_chat_model(model_name: str, cache_quant: int) -> bool:
    global loaded_model
    if loaded_model is not None:
        del loaded_model
        loaded_model = None
    loaded_model = ChatModel(model_name, cache_quant)
    return True

def get_loaded_chat_model_name(blank_arg: bool) -> str:
    global loaded_model
    if loaded_model is None:
        return ""
    return loaded_model.name

def send_chat(username: str, message: str) -> str:
    if loaded_model is None:
        return "No model is loaded"
    return loaded_model.send_chat(username, message)

def send_chat_stream(username: str, message: str) -> Generator[str, None, None]:
    return loaded_model.send_chat_stream(username, message)

