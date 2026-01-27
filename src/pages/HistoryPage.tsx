import { useEffect, useState } from "react";
import { format } from "date-fns";
import { useAppStore } from "../stores/appStore";
import * as api from "../lib/api";
import type { ExportFormat } from "../types";
import clsx from "clsx";

export default function HistoryPage() {
  const {
    history,
    selectedHistoryItem,
    modes,
    loadHistory,
    selectHistoryItem,
    reprocessHistoryItem,
    deleteHistoryItem,
  } = useAppStore();

  const [searchQuery, setSearchQuery] = useState("");
  const [reprocessMode, setReprocessMode] = useState("");

  useEffect(() => {
    loadHistory();
  }, [loadHistory]);

  const handleSearch = () => {
    loadHistory(searchQuery || undefined);
  };

  const handleExport = async (format: ExportFormat) => {
    if (!selectedHistoryItem) return;

    try {
      const content = await api.exportHistoryItem(selectedHistoryItem.id, format);
      const blob = new Blob([content], { type: "text/plain" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `transcription-${selectedHistoryItem.id}.${format}`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error("Export failed:", error);
    }
  };

  const handleReprocess = async () => {
    if (!selectedHistoryItem || !reprocessMode) return;
    await reprocessHistoryItem(selectedHistoryItem.id, reprocessMode);
    setReprocessMode("");
  };

  const handleDelete = async () => {
    if (!selectedHistoryItem) return;
    if (confirm("Are you sure you want to delete this item?")) {
      await deleteHistoryItem(selectedHistoryItem.id);
    }
  };

  const formatDuration = (ms: number) => {
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
  };

  return (
    <div className="flex gap-4 h-[calc(100vh-200px)]">
      {/* History list */}
      <div className="w-1/3 flex flex-col">
        {/* Search */}
        <div className="flex gap-2 mb-4">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            placeholder="Search transcriptions..."
            className="flex-1 bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={handleSearch}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg text-sm hover:bg-blue-700"
          >
            Search
          </button>
        </div>

        {/* List */}
        <div className="flex-1 overflow-auto space-y-2">
          {history.length === 0 ? (
            <p className="text-gray-500 text-center py-8">No history yet</p>
          ) : (
            history.map((item) => (
              <button
                key={item.id}
                onClick={() => selectHistoryItem(item)}
                className={clsx(
                  "w-full text-left p-3 rounded-lg transition-colors",
                  selectedHistoryItem?.id === item.id
                    ? "bg-blue-600"
                    : "bg-gray-800 hover:bg-gray-700"
                )}
              >
                <div className="flex items-center justify-between mb-1">
                  <span className="text-xs text-gray-400">
                    {format(new Date(item.created_at), "MMM d, h:mm a")}
                  </span>
                  <span className="text-xs text-gray-500">
                    {formatDuration(item.duration_ms)}
                  </span>
                </div>
                <p className="text-sm text-white truncate">
                  {item.output_final.substring(0, 100)}
                </p>
                <div className="flex items-center gap-2 mt-1">
                  <span className="text-xs px-2 py-0.5 bg-gray-700 rounded">
                    {item.mode_key}
                  </span>
                  {item.error && (
                    <span className="text-xs text-red-400">Error</span>
                  )}
                </div>
              </button>
            ))
          )}
        </div>
      </div>

      {/* Detail view */}
      <div className="flex-1 bg-gray-800 rounded-lg p-4">
        {selectedHistoryItem ? (
          <div className="h-full flex flex-col">
            {/* Header */}
            <div className="flex items-center justify-between mb-4">
              <div>
                <h2 className="text-lg font-medium text-white">
                  {format(
                    new Date(selectedHistoryItem.created_at),
                    "MMMM d, yyyy 'at' h:mm a"
                  )}
                </h2>
                <p className="text-sm text-gray-400">
                  Mode: {selectedHistoryItem.mode_key} • Duration:{" "}
                  {formatDuration(selectedHistoryItem.duration_ms)} • STT:{" "}
                  {selectedHistoryItem.stt_provider}/{selectedHistoryItem.stt_model}
                  {selectedHistoryItem.llm_provider &&
                    ` • LLM: ${selectedHistoryItem.llm_provider}/${selectedHistoryItem.llm_model}`}
                </p>
              </div>
              <button
                onClick={handleDelete}
                className="text-red-400 hover:text-red-300 text-sm"
              >
                Delete
              </button>
            </div>

            {/* Content */}
            <div className="flex-1 overflow-auto space-y-4">
              {/* Raw transcript */}
              <div>
                <h3 className="text-sm font-medium text-gray-400 mb-2">
                  Raw Transcript
                </h3>
                <div className="bg-gray-900 rounded p-3 text-sm text-gray-300 whitespace-pre-wrap">
                  {selectedHistoryItem.transcript_raw}
                </div>
              </div>

              {/* Final output */}
              <div>
                <div className="flex items-center justify-between mb-2">
                  <h3 className="text-sm font-medium text-gray-400">
                    Final Output
                  </h3>
                  <button
                    onClick={() =>
                      navigator.clipboard.writeText(
                        selectedHistoryItem.output_final
                      )
                    }
                    className="text-xs text-blue-400 hover:text-blue-300"
                  >
                    Copy
                  </button>
                </div>
                <div className="bg-gray-900 rounded p-3 text-sm text-white whitespace-pre-wrap">
                  {selectedHistoryItem.output_final}
                </div>
              </div>
            </div>

            {/* Actions */}
            <div className="mt-4 pt-4 border-t border-gray-700 flex items-center gap-4">
              {/* Reprocess */}
              <div className="flex items-center gap-2">
                <select
                  value={reprocessMode}
                  onChange={(e) => setReprocessMode(e.target.value)}
                  className="bg-gray-700 border border-gray-600 rounded px-2 py-1 text-sm text-white"
                >
                  <option value="">Reprocess with...</option>
                  {modes.map((mode) => (
                    <option key={mode.key} value={mode.key}>
                      {mode.name}
                    </option>
                  ))}
                </select>
                <button
                  onClick={handleReprocess}
                  disabled={!reprocessMode}
                  className="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Reprocess
                </button>
              </div>

              {/* Export */}
              <div className="flex items-center gap-2">
                <span className="text-sm text-gray-400">Export:</span>
                {(["txt", "md", "srt", "vtt"] as ExportFormat[]).map((fmt) => (
                  <button
                    key={fmt}
                    onClick={() => handleExport(fmt)}
                    className="px-2 py-1 bg-gray-700 text-gray-300 rounded text-xs hover:bg-gray-600"
                  >
                    .{fmt}
                  </button>
                ))}
              </div>
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-500">
            Select an item to view details
          </div>
        )}
      </div>
    </div>
  );
}
