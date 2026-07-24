import { describe, it, expect } from 'vitest'

function calculateCost(provider: string, model: string, promptTokens: number, completionTokens: number): number {
  const configs: Record<string, { promptPrice: number; completionPrice: number }> = {
    openai: { promptPrice: 0.0000015, completionPrice: 0.000002 },
    deepseek: { promptPrice: 0.000002, completionPrice: 0.000006 },
    qwen: { promptPrice: 0.0000015, completionPrice: 0.0000045 },
    anthropic: { promptPrice: 0.00003, completionPrice: 0.00015 },
    gemini: { promptPrice: 0.000001, completionPrice: 0.0000015 },
    doubao: { promptPrice: 0.000001, completionPrice: 0.000003 },
    yiyan: { promptPrice: 0.000001, completionPrice: 0.000002 },
    openrouter: { promptPrice: 0.000001, completionPrice: 0.000002 },
    groq: { promptPrice: 0.0000005, completionPrice: 0.0000015 },
    mistral: { promptPrice: 0.0000002, completionPrice: 0.0000006 },
    together: { promptPrice: 0.000001, completionPrice: 0.000003 },
    replicate: { promptPrice: 0.000002, completionPrice: 0.000006 },
    huggingface: { promptPrice: 0.000001, completionPrice: 0.000003 },
  }

  let config = configs[provider] || configs.openai
  let { promptPrice, completionPrice } = config

  if (provider === 'openai') {
    if (model.includes('gpt-4o')) {
      promptPrice = 0.000005
      completionPrice = 0.000015
    } else if (model.includes('gpt-4')) {
      promptPrice = 0.00003
      completionPrice = 0.00006
    }
  } else if (provider === 'anthropic') {
    if (model.includes('claude-3-opus')) {
      promptPrice = 0.00015
      completionPrice = 0.00075
    } else if (model.includes('claude-3-sonnet')) {
      promptPrice = 0.00003
      completionPrice = 0.00015
    }
  } else if (provider === 'gemini') {
    if (model.includes('ultra')) {
      promptPrice = 0.0000125
      completionPrice = 0.0000375
    }
  }

  return (promptTokens * promptPrice) + (completionTokens * completionPrice)
}

describe('Cost Calculator', () => {
  it('should calculate cost for OpenAI GPT-3.5', () => {
    const cost = calculateCost('openai', 'gpt-3.5-turbo', 1000, 500)
    expect(cost).toBeCloseTo(0.0025, 6)
  })

  it('should calculate cost for OpenAI GPT-4', () => {
    const cost = calculateCost('openai', 'gpt-4', 1000, 500)
    expect(cost).toBeCloseTo(0.06, 4)
  })

  it('should calculate cost for OpenAI GPT-4o', () => {
    const cost = calculateCost('openai', 'gpt-4o', 1000, 500)
    expect(cost).toBeCloseTo(0.0125, 4)
  })

  it('should calculate cost for Anthropic Claude-3-Sonnet', () => {
    const cost = calculateCost('anthropic', 'claude-3-sonnet', 1000, 500)
    expect(cost).toBeCloseTo(0.105, 4)
  })

  it('should calculate cost for Anthropic Claude-3-Opus', () => {
    const cost = calculateCost('anthropic', 'claude-3-opus', 1000, 500)
    expect(cost).toBeCloseTo(0.525, 4)
  })

  it('should calculate cost for Gemini Ultra', () => {
    const cost = calculateCost('gemini', 'gemini-ultra', 1000, 500)
    expect(cost).toBeCloseTo(0.03125, 5)
  })

  it('should calculate cost for DeepSeek', () => {
    const cost = calculateCost('deepseek', 'deepseek-chat', 1000, 500)
    expect(cost).toBeCloseTo(0.005, 5)
  })

  it('should return 0 for zero tokens', () => {
    const cost = calculateCost('openai', 'gpt-3.5-turbo', 0, 0)
    expect(cost).toBe(0)
  })

  it('should handle unknown provider with default price', () => {
    const cost = calculateCost('unknown', 'model', 1000, 500)
    expect(cost).toBeCloseTo(0.0025, 6)
  })
})

function formatCurrency(value: number, decimals: number = 4): string {
  return value.toFixed(decimals)
}

function maskApiKey(apiKey: string, visibleLength: number = 4): string {
  if (apiKey.length <= visibleLength * 2) return apiKey
  const prefix = apiKey.slice(0, visibleLength)
  const suffix = apiKey.slice(-visibleLength)
  return `${prefix}***${suffix}`
}

describe('Data Formatter', () => {
  it('should format currency correctly', () => {
    expect(formatCurrency(0.00123456)).toBe('0.0012')
    expect(formatCurrency(123.45)).toBe('123.4500')
    expect(formatCurrency(0)).toBe('0.0000')
  })

  it('should mask API key correctly', () => {
    expect(maskApiKey('sk-abcdefghijklmnopqrstuvwxyz', 4)).toBe('sk-a***wxyz')
    expect(maskApiKey('short', 4)).toBe('short')
    expect(maskApiKey('verylongapikey123456', 4)).toBe('very***3456')
  })
})
