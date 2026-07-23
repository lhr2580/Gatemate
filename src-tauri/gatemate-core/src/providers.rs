use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProviderConfig {
    pub name: &'static str,
    pub url: &'static str,
    pub auth_header: &'static str,
    pub auth_prefix: &'static str,
    pub prompt_price: f64,
    pub completion_price: f64,
}

pub static PROVIDERS: [(&'static str, ProviderConfig); 13] = [
    ("openai", ProviderConfig {
        name: "OpenAI",
        url: "https://api.openai.com/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.0000015,
        completion_price: 0.000002,
    }),
    ("deepseek", ProviderConfig {
        name: "DeepSeek",
        url: "https://api.deepseek.com/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.000002,
        completion_price: 0.000006,
    }),
    ("qwen", ProviderConfig {
        name: "Qwen",
        url: "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.0000015,
        completion_price: 0.0000045,
    }),
    ("anthropic", ProviderConfig {
        name: "Anthropic",
        url: "https://api.anthropic.com/v1/messages",
        auth_header: "x-api-key",
        auth_prefix: "",
        prompt_price: 0.00003,
        completion_price: 0.00015,
    }),
    ("gemini", ProviderConfig {
        name: "Gemini",
        url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent",
        auth_header: "x-goog-api-key",
        auth_prefix: "",
        prompt_price: 0.000001,
        completion_price: 0.0000015,
    }),
    ("doubao", ProviderConfig {
        name: "Doubao",
        url: "https://api.doubao.com/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.000001,
        completion_price: 0.000003,
    }),
    ("yiyan", ProviderConfig {
        name: "YiYan",
        url: "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.000001,
        completion_price: 0.000002,
    }),
    ("openrouter", ProviderConfig {
        name: "OpenRouter",
        url: "https://openrouter.ai/api/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.000001,
        completion_price: 0.000002,
    }),
    ("groq", ProviderConfig {
        name: "Groq",
        url: "https://api.groq.com/openai/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.0000005,
        completion_price: 0.0000015,
    }),
    ("mistral", ProviderConfig {
        name: "Mistral",
        url: "https://api.mistral.ai/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.0000002,
        completion_price: 0.0000006,
    }),
    ("together", ProviderConfig {
        name: "Together AI",
        url: "https://api.together.xyz/v1/chat/completions",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.000001,
        completion_price: 0.000003,
    }),
    ("replicate", ProviderConfig {
        name: "Replicate",
        url: "https://api.replicate.com/v1/completions",
        auth_header: "Authorization",
        auth_prefix: "Token ",
        prompt_price: 0.000002,
        completion_price: 0.000006,
    }),
    ("huggingface", ProviderConfig {
        name: "Hugging Face",
        url: "https://api-inference.huggingface.co/models/",
        auth_header: "Authorization",
        auth_prefix: "Bearer ",
        prompt_price: 0.000001,
        completion_price: 0.000003,
    }),
];

pub fn get_provider_config(provider: &str) -> Option<&'static ProviderConfig> {
    PROVIDERS.iter().find(|(name, _)| *name == provider).map(|(_, config)| config)
}

pub fn get_provider_url(provider: &str) -> String {
    get_provider_config(provider)
        .map(|c| c.url.to_string())
        .unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".to_string())
}

pub fn get_provider_url_with_model(provider: &str, model: &str) -> String {
    let base_url = get_provider_url(provider);
    if provider == "huggingface" && !model.is_empty() {
        base_url + model
    } else {
        base_url
    }
}

pub fn calculate_cost(provider: &str, model: &str, prompt_tokens: i64, completion_tokens: i64) -> f64 {
    let config = match get_provider_config(provider) {
        Some(c) => c,
        None => return (prompt_tokens as f64 * 0.000001) + (completion_tokens as f64 * 0.000002),
    };
    
    let (prompt_price, completion_price) = match provider {
        "openai" => {
            if model.contains("gpt-4") {
                (0.00003, 0.00006)
            } else if model.contains("gpt-4o") {
                (0.000005, 0.000015)
            } else {
                (config.prompt_price, config.completion_price)
            }
        }
        "deepseek" => {
            if model.contains("chat") {
                (config.prompt_price, config.completion_price)
            } else {
                (0.0000015, 0.0000045)
            }
        }
        "qwen" => {
            if model.contains("plus") {
                (0.000003, 0.000009)
            } else {
                (config.prompt_price, config.completion_price)
            }
        }
        "anthropic" => {
            if model.contains("claude-3-opus") {
                (0.00015, 0.00075)
            } else if model.contains("claude-3-sonnet") {
                (0.00003, 0.00015)
            } else {
                (config.prompt_price, config.completion_price)
            }
        }
        "gemini" => {
            if model.contains("ultra") {
                (0.0000125, 0.0000375)
            } else {
                (config.prompt_price, config.completion_price)
            }
        }
        "mistral" => {
            if model.contains("large") {
                (0.000002, 0.000006)
            } else {
                (config.prompt_price, config.completion_price)
            }
        }
        _ => (config.prompt_price, config.completion_price),
    };
    
    (prompt_tokens as f64 * prompt_price) + (completion_tokens as f64 * completion_price)
}