import { describe, it, expect } from "vitest";
import type { Mode, HistoryItem, Settings } from "../types";

describe("Types", () => {
  it("Mode type should have required fields", () => {
    const mode: Mode = {
      key: "test",
      name: "Test Mode",
      description: "A test mode",
      stt_provider: "whispercpp",
      stt_model: "base.en",
      ai_processing: false,
      llm_provider: "ollama",
      llm_model: "",
      prompt_template: "",
      output_format: "plain",
      builtin: true,
    };

    expect(mode.key).toBe("test");
    expect(mode.ai_processing).toBe(false);
  });

  it("HistoryItem type should have required fields", () => {
    const item: HistoryItem = {
      id: "123",
      created_at: "2024-01-01T00:00:00Z",
      mode_key: "voice_to_text",
      audio_path: null,
      transcript_raw: "Hello world",
      output_final: "Hello world",
      stt_provider: "whispercpp",
      stt_model: "base.en",
      llm_provider: null,
      llm_model: null,
      duration_ms: 1000,
      error: null,
    };

    expect(item.id).toBe("123");
    expect(item.duration_ms).toBe(1000);
  });

  it("Settings type should have default values structure", () => {
    const settings: Settings = {
      default_stt_provider: "whispercpp",
      default_stt_model: "base.en",
      default_llm_provider: "ollama",
      default_llm_model: "llama3.2",
      active_mode_key: "voice_to_text",
      input_device: "",
      auto_paste: true,
      context_awareness: false,
      language: "en",
    };

    expect(settings.auto_paste).toBe(true);
    expect(settings.language).toBe("en");
  });
});
