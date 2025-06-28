import React, { useState } from "react";
import { greet } from "../services/backendService";

interface GreetingViewProps {
  onError?: (error: string) => void;
  setLoading?: (loading: boolean) => void;
}

export function GreetingView({
  onError,
  setLoading,
}: GreetingViewProps): React.JSX.Element {
  const [name, setName] = useState("");
  const [greeting, setGreeting] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;

    try {
      setLoading?.(true);
      const result = await greet(name);
      setGreeting(result);
      setName("");
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Failed to greet";
      onError?.(errorMessage);
    } finally {
      setLoading?.(false);
    }
  };

  return (
    <div className="mx-auto max-w-md rounded-lg bg-white p-8 shadow-lg">
      <h2 className="mb-6 text-center text-2xl font-bold text-gray-800">
        Greeting Demo
      </h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label
            htmlFor="name"
            className="mb-2 block text-sm font-medium text-gray-700"
          >
            Your Name
          </label>
          <input
            type="text"
            id="name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Enter your name"
            className="w-full rounded-md border border-gray-300 px-3 py-2 focus:border-transparent focus:ring-2 focus:ring-blue-500"
            required
          />
        </div>

        <button
          type="submit"
          className="w-full rounded-md bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={!name.trim()}
        >
          Get Greeting
        </button>
      </form>

      {greeting && (
        <div className="mt-6 rounded-lg border border-green-200 bg-green-50 p-4">
          <p className="text-center font-medium text-green-800">{greeting}</p>
        </div>
      )}
    </div>
  );
}
