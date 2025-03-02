from langchain_ollama import ChatOllama

chat_model = ChatOllama(model="hf.co/cognitivecomputations/Dolphin3.0-Llama3.1-8B-GGUF:Q4_0")

def call_dolphin_llama_3() -> ChatOllama:
    return chat_model