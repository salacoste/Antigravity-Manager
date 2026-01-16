// OpenAI 协议响应转换模块
use super::models::*;
use serde_json::Value;

pub fn transform_openai_response(gemini_response: &Value) -> OpenAIResponse {
    // 解包 response 字段
    let raw = gemini_response.get("response").unwrap_or(gemini_response);

    let mut choices = Vec::new();

    // 支持多候选结果 (n > 1)
    if let Some(candidates) = raw.get("candidates").and_then(|c| c.as_array()) {
        for (idx, candidate) in candidates.iter().enumerate() {
            let mut content_out = String::new();
            let mut thought_out = String::new();
            let mut tool_calls = Vec::new();

            // 提取 content 和 tool_calls
            if let Some(parts) = candidate
                .get("content")
                .and_then(|c| c.get("parts"))
                .and_then(|p| p.as_array())
            {
                for part in parts {
                    // 捕获 thoughtSignature (Gemini 3 工具调用必需)
                    if let Some(sig) = part
                        .get("thoughtSignature")
                        .or(part.get("thought_signature"))
                        .and_then(|s| s.as_str())
                    {
                        super::streaming::store_thought_signature(sig);
                    }

                    // 检查该 part 是否是思考内容 (thought: true)
                    let is_thought_part = part
                        .get("thought")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    // 文本部分
                    if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                        if is_thought_part {
                            // thought: true 时，text 是思考内容
                            thought_out.push_str(text);
                        } else {
                            // 正常内容
                            content_out.push_str(text);
                        }
                    }

                    // 工具调用部分
                    if let Some(fc) = part.get("functionCall") {
                        let name = fc.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                        let args = fc
                            .get("args")
                            .map_or_else(|| "{}".to_string(), |v| v.to_string());
                        let id = fc.get("id").and_then(|v| v.as_str()).map_or_else(
                            || format!("{}-{}", name, uuid::Uuid::new_v4()),
                            |s| s.to_string(),
                        );

                        tool_calls.push(ToolCall {
                            id,
                            r#type: "function".to_string(),
                            function: ToolFunction {
                                name: name.to_string(),
                                arguments: args,
                            },
                        });
                    }

                    // 图片处理 (响应中直接返回图片的情况)
                    if let Some(img) = part.get("inlineData") {
                        let mime_type = img
                            .get("mimeType")
                            .and_then(|v| v.as_str())
                            .unwrap_or("image/png");
                        let data = img.get("data").and_then(|v| v.as_str()).unwrap_or("");
                        if !data.is_empty() {
                            content_out
                                .push_str(&format!("![image](data:{};base64,{})", mime_type, data));
                        }
                    }
                }
            }

            // 提取并处理该候选结果的联网搜索引文 (Grounding Metadata)
            if let Some(grounding) = candidate.get("groundingMetadata") {
                let mut grounding_text = String::new();

                // 1. 处理搜索词
                if let Some(queries) = grounding.get("webSearchQueries").and_then(|q| q.as_array())
                {
                    let query_list: Vec<&str> = queries.iter().filter_map(|v| v.as_str()).collect();
                    if !query_list.is_empty() {
                        grounding_text.push_str("\n\n---\n**🔍 已为您搜索：** ");
                        grounding_text.push_str(&query_list.join(", "));
                    }
                }

                // 2. 处理来源链接 (Chunks)
                if let Some(chunks) = grounding.get("groundingChunks").and_then(|c| c.as_array()) {
                    let mut links = Vec::new();
                    for (i, chunk) in chunks.iter().enumerate() {
                        if let Some(web) = chunk.get("web") {
                            let title = web
                                .get("title")
                                .and_then(|v| v.as_str())
                                .unwrap_or("网页来源");
                            let uri = web.get("uri").and_then(|v| v.as_str()).unwrap_or("#");
                            links.push(format!("[{}] [{}]({})", i + 1, title, uri));
                        }
                    }

                    if !links.is_empty() {
                        grounding_text.push_str("\n\n**🌐 来源引文：**\n");
                        grounding_text.push_str(&links.join("\n"));
                    }
                }

                if !grounding_text.is_empty() {
                    content_out.push_str(&grounding_text);
                }
            }

            // 提取该候选结果的 finish_reason
            let finish_reason = candidate
                .get("finishReason")
                .and_then(|f| f.as_str())
                .map_or("stop", |f| match f {
                    "STOP" => "stop",
                    "MAX_TOKENS" => "length",
                    "SAFETY" => "content_filter",
                    "RECITATION" => "content_filter",
                    _ => "stop",
                });

            choices.push(Choice {
                index: idx as u32,
                message: OpenAIMessage {
                    role: "assistant".to_string(),
                    content: if content_out.is_empty() {
                        None
                    } else {
                        Some(OpenAIContent::String(content_out))
                    },
                    reasoning_content: if thought_out.is_empty() {
                        None
                    } else {
                        Some(thought_out)
                    },
                    tool_calls: if tool_calls.is_empty() {
                        None
                    } else {
                        Some(tool_calls)
                    },
                    tool_call_id: None,
                    name: None,
                },
                finish_reason: Some(finish_reason.to_string()),
            });
        }
    }

    // Extract and map usage metadata from Gemini to OpenAI format
    let usage = raw.get("usageMetadata").and_then(|u| {
        let prompt_tokens = u
            .get("promptTokenCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let completion_tokens = u
            .get("candidatesTokenCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let total_tokens = u
            .get("totalTokenCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let cached_tokens = u
            .get("cachedContentTokenCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        Some(super::models::OpenAIUsage {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            prompt_tokens_details: cached_tokens.map(|ct| super::models::PromptTokensDetails {
                cached_tokens: Some(ct),
            }),
            completion_tokens_details: None,
        })
    });

    OpenAIResponse {
        id: raw
            .get("responseId")
            .and_then(|v| v.as_str())
            .unwrap_or("resp_unknown")
            .to_string(),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp() as u64,
        model: raw
            .get("modelVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        choices,
        usage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_transform_openai_response() {
        let gemini_resp = json!({
            "candidates": [{
                "content": {
                    "parts": [{"text": "Hello!"}]
                },
                "finishReason": "STOP"
            }],
            "modelVersion": "gemini-2.5-pro",
            "responseId": "resp_123"
        });

        let result = transform_openai_response(&gemini_resp);
        assert_eq!(result.object, "chat.completion");
        let content = match result.choices[0].message.content.as_ref().unwrap() {
            OpenAIContent::String(s) => s,
            _ => panic!("Expected string content"),
        };
        assert_eq!(content, "Hello!");
        assert_eq!(result.choices[0].finish_reason, Some("stop".to_string()));
    }

    #[test]
    fn test_usage_metadata_mapping() {
        let gemini_resp = json!({
            "candidates": [{
                "content": {"parts": [{"text": "Hello!"}]},
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 100,
                "candidatesTokenCount": 50,
                "totalTokenCount": 150,
                "cachedContentTokenCount": 25
            },
            "modelVersion": "gemini-2.5-pro",
            "responseId": "resp_123"
        });

        let result = transform_openai_response(&gemini_resp);
        
        assert!(result.usage.is_some());
        let usage = result.usage.unwrap();
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
        assert!(usage.prompt_tokens_details.is_some());
        assert_eq!(usage.prompt_tokens_details.unwrap().cached_tokens, Some(25));
    }

    #[test]
    fn test_response_without_usage_metadata() {
        let gemini_resp = json!({
            "candidates": [{
                "content": {"parts": [{"text": "Hello!"}]},
                "finishReason": "STOP"
            }],
            "modelVersion": "gemini-2.5-pro",
            "responseId": "resp_123"
        });

        let result = transform_openai_response(&gemini_resp);
        assert!(result.usage.is_none());
    }
}
