import React, { useState, useEffect } from "react";
import { getCount, increment, setCount } from "../services/backendService";

interface CounterViewProps {
  onError?: (error: string) => void;
  setLoading?: (loading: boolean) => void;
}

export function CounterView({
  onError,
  setLoading,
}: CounterViewProps): React.JSX.Element {
  const [count, setCountState] = useState<bigint>(0n);
  const [customValue, setCustomValue] = useState("");

  useEffect(() => {
    loadCount();
  }, []);

  const loadCount = async () => {
    try {
      setLoading?.(true);
      const currentCount = await getCount();
      setCountState(currentCount);
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Failed to load count";
      onError?.(errorMessage);
    } finally {
      setLoading?.(false);
    }
  };

  const handleIncrement = async () => {
    try {
      setLoading?.(true);
      const newCount = await increment();
      setCountState(newCount);
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Failed to increment";
      onError?.(errorMessage);
    } finally {
      setLoading?.(false);
    }
  };

  const handleSetCount = async (e: React.FormEvent) => {
    e.preventDefault();
    const value = customValue.trim();
    if (!value) return;

    try {
      setLoading?.(true);
      const bigintValue = BigInt(value);
      const newCount = await setCount(bigintValue);
      setCountState(newCount);
      setCustomValue("");
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Failed to set count";
      onError?.(errorMessage);
    } finally {
      setLoading?.(false);
    }
  };

  return (
    <div className="mx-auto max-w-md rounded-lg bg-white p-8 shadow-lg">
      <h2 className="mb-6 text-center text-2xl font-bold text-gray-800">
        Counter Demo
      </h2>

      <div className="mb-8 text-center">
        <div className="mb-2 text-6xl font-bold text-blue-600">
          {count.toString()}
        </div>
        <p className="text-gray-600">Current Count</p>
      </div>

      <div className="space-y-4">
        <button
          onClick={handleIncrement}
          className="w-full rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-blue-700"
        >
          Increment (+1)
        </button>

        <div className="border-t pt-4">
          <form onSubmit={handleSetCount} className="flex gap-2">
            <input
              type="number"
              value={customValue}
              onChange={(e) => setCustomValue(e.target.value)}
              placeholder="Enter number"
              className="flex-1 rounded-md border border-gray-300 px-3 py-2 focus:border-transparent focus:ring-2 focus:ring-blue-500"
            />
            <button
              type="submit"
              className="rounded-md bg-green-600 px-4 py-2 text-white transition-colors hover:bg-green-700"
            >
              Set
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
