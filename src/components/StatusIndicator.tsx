import clsx from "clsx";
import type { RecordingStatus } from "../types";

interface StatusIndicatorProps {
  status: RecordingStatus;
  size?: "sm" | "md" | "lg";
}

const statusConfig: Record<
  RecordingStatus,
  { color: string; label: string; animate: boolean }
> = {
  loading: {
    color: "bg-yellow-500",
    label: "Loading model...",
    animate: true,
  },
  recording: {
    color: "bg-red-500",
    label: "Recording",
    animate: true,
  },
  processing: {
    color: "bg-blue-500",
    label: "Processing...",
    animate: true,
  },
  ready: {
    color: "bg-green-500",
    label: "Ready",
    animate: false,
  },
  error: {
    color: "bg-red-700",
    label: "Error",
    animate: false,
  },
};

const sizeClasses = {
  sm: "w-2 h-2",
  md: "w-3 h-3",
  lg: "w-4 h-4",
};

export default function StatusIndicator({
  status,
  size = "md",
}: StatusIndicatorProps) {
  const config = statusConfig[status];

  return (
    <div className="flex items-center gap-2">
      <div
        className={clsx(
          "rounded-full",
          config.color,
          sizeClasses[size],
          config.animate && "animate-pulse"
        )}
      />
      <span className="text-sm text-gray-300">{config.label}</span>
    </div>
  );
}
