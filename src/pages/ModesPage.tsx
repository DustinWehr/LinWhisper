import { useAppStore } from "../stores/appStore";
import clsx from "clsx";

export default function ModesPage() {
  const { modes, activeMode, setActiveMode } = useAppStore();

  const modesList = Object.values(modes);
  const builtinModes = modesList.filter((m) => m.builtin);
  const customModes = modesList.filter((m) => !m.builtin);

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-white">Modes</h1>
        <button className="px-4 py-2 bg-blue-600 text-white rounded-lg text-sm hover:bg-blue-700">
          + Create Mode
        </button>
      </div>

      {/* Built-in modes */}
      <section>
        <h2 className="text-lg font-medium text-gray-300 mb-3">Built-in Modes</h2>
        <div className="grid gap-3">
          {builtinModes.map((mode) => (
            <ModeCard
              key={mode.key}
              mode={mode}
              isActive={activeMode?.key === mode.key}
              onActivate={() => setActiveMode(mode.key)}
            />
          ))}
        </div>
      </section>

      {/* Custom modes */}
      {customModes.length > 0 && (
        <section>
          <h2 className="text-lg font-medium text-gray-300 mb-3">Custom Modes</h2>
          <div className="grid gap-3">
            {customModes.map((mode) => (
              <ModeCard
                key={mode.key}
                mode={mode}
                isActive={activeMode?.key === mode.key}
                onActivate={() => setActiveMode(mode.key)}
                editable
              />
            ))}
          </div>
        </section>
      )}

      {/* Mode format info */}
      <section className="bg-gray-800/50 rounded-lg p-4">
        <h3 className="text-sm font-medium text-gray-300 mb-2">
          Creating Custom Modes
        </h3>
        <p className="text-sm text-gray-400 mb-3">
          Custom modes are stored as JSON files in{" "}
          <code className="bg-gray-700 px-1 rounded">
            ~/.config/whispertray/modes/
          </code>
        </p>
        <pre className="bg-gray-900 rounded p-3 text-xs text-gray-300 overflow-auto">
{`{
  "key": "my_custom_mode",
  "name": "My Custom Mode",
  "description": "Description of what this mode does",
  "stt_provider": "whispercpp",
  "stt_model": "base.en",
  "ai_processing": true,
  "llm_provider": "ollama",
  "llm_model": "llama3.2",
  "prompt_template": "Your prompt here. Use {{transcript}} for the transcribed text.",
  "output_format": "plain"
}`}
        </pre>
        <p className="text-xs text-gray-500 mt-2">
          Available template variables: <code>{"{{transcript}}"}</code>,{" "}
          <code>{"{{context}}"}</code>, <code>{"{{language}}"}</code>
        </p>
      </section>
    </div>
  );
}

interface ModeCardProps {
  mode: {
    key: string;
    name: string;
    description: string;
    ai_processing: boolean;
    stt_provider: string;
    stt_model: string;
    llm_provider: string;
    llm_model: string;
  };
  isActive: boolean;
  onActivate: () => void;
  editable?: boolean;
}

function ModeCard({ mode, isActive, onActivate, editable }: ModeCardProps) {
  return (
    <div
      className={clsx(
        "bg-gray-800 rounded-lg p-4 border-2 transition-colors",
        isActive ? "border-blue-500" : "border-transparent hover:border-gray-600"
      )}
    >
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <h3 className="font-medium text-white">{mode.name}</h3>
            {isActive && (
              <span className="text-xs bg-blue-600 text-white px-2 py-0.5 rounded">
                Active
              </span>
            )}
            {mode.ai_processing && (
              <span className="text-xs bg-purple-600 text-white px-2 py-0.5 rounded">
                AI
              </span>
            )}
          </div>
          <p className="text-sm text-gray-400 mt-1">{mode.description}</p>
          <div className="flex items-center gap-4 mt-2 text-xs text-gray-500">
            <span>
              STT: {mode.stt_provider}/{mode.stt_model}
            </span>
            {mode.ai_processing && (
              <span>
                LLM: {mode.llm_provider}/{mode.llm_model}
              </span>
            )}
          </div>
        </div>
        <div className="flex items-center gap-2">
          {!isActive && (
            <button
              onClick={onActivate}
              className="px-3 py-1 bg-gray-700 text-white rounded text-sm hover:bg-gray-600"
            >
              Activate
            </button>
          )}
          {editable && (
            <button className="px-3 py-1 bg-gray-700 text-white rounded text-sm hover:bg-gray-600">
              Edit
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
