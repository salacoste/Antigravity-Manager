#!/usr/bin/env python3
"""
测试 Gemini 图片生成功能
复现 Issue #581 中报告的 429 错误问题
"""

import os
from openai import OpenAI

# 配置
API_KEY = "sk-79754f1fecab4451aa8bb9f0ba4d5ee9"
BASE_URL = "http://127.0.0.1:8045/v1"

# 测试的模型列表
MODELS = [
    "gemini-3-pro-image-16x9-4k",
    "gemini-3-pro-image-4k", 
    "gemini-3-pro-image"
]

# 测试提示词
PROMPT = "A Super Saiyan warrior with golden hair, intense battle scene, masterpiece, best quality, ultra-detailed"

def test_image_generation(model: str):
    """测试单个模型的图片生成"""
    print(f"\n{'='*60}")
    print(f"测试模型: {model}")
    print(f"{'='*60}")
    
    try:
        client = OpenAI(
            api_key=API_KEY,
            base_url=BASE_URL
        )
        
        print(f"发送请求...")
        response = client.images.generate(
            model=model,
            prompt=PROMPT,
            n=1,
            size="1024x1024"
        )
        
        print(f"✅ 成功! 生成了 {len(response.data)} 张图片")
        
        # 打印响应详情
        print(f"\n响应对象类型: {type(response)}")
        print(f"响应数据: {response.data}")
        
        for idx, image in enumerate(response.data):
            print(f"\n图片 {idx + 1}:")
            print(f"  - URL: {image.url if image.url else 'None'}")
            print(f"  - Revised Prompt: {image.revised_prompt if hasattr(image, 'revised_prompt') else 'N/A'}")
            print(f"  - b64_json: {('存在' if image.b64_json else '无') if hasattr(image, 'b64_json') else 'N/A'}")
            
        return True
        
    except Exception as e:
        print(f"❌ 失败!")
        print(f"错误类型: {type(e).__name__}")
        print(f"错误信息: {str(e)}")
        
        # 打印完整的异常堆栈
        import traceback
        print(f"\n完整错误堆栈:")
        traceback.print_exc()
        
        # 检查是否是 429 错误
        if "429" in str(e):
            print("\n⚠️  检测到 429 错误 (Too Many Requests)")
        
        return False

def main():
    print("="*60)
    print("Antigravity Manager - 图片生成测试")
    print("Issue #581 复现测试")
    print("="*60)
    print(f"API Key: {API_KEY}")
    print(f"Base URL: {BASE_URL}")
    print(f"测试提示词: {PROMPT}")
    
    results = {}
    
    for model in MODELS:
        success = test_image_generation(model)
        results[model] = success
    
    # 总结
    print(f"\n{'='*60}")
    print("测试总结")
    print(f"{'='*60}")
    
    success_count = sum(1 for v in results.values() if v)
    total_count = len(results)
    
    for model, success in results.items():
        status = "✅ 成功" if success else "❌ 失败"
        print(f"{status} - {model}")
    
    print(f"\n成功率: {success_count}/{total_count}")
    
    if success_count == 0:
        print("\n⚠️  所有模型都失败了，可能确认了 Issue #581 的问题")
    elif success_count < total_count:
        print("\n⚠️  部分模型失败，可能是特定模型的问题")
    else:
        print("\n✅ 所有模型都成功，无法复现 Issue #581")

if __name__ == "__main__":
    main()
