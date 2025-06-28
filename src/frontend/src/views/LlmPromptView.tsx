import React, { useState } from "react";
import { chat } from "../services/backendService";

// Define ChatMessage interface locally since the import might be causing issues
interface ChatMessage {
  role: string;
  content: string;
}

interface LlmPromptViewProps {
  onError?: (error: string) => void;
  setLoading?: (loading: boolean) => void;
}

export function LlmPromptView({
  onError,
  setLoading,
}: LlmPromptViewProps): React.JSX.Element {
  const [prompt, setPrompt] = useState("");
  const [conversation, setConversation] = useState<ChatMessage[]>([]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!prompt.trim()) return;

    const userMessage: ChatMessage = {
      role: "user",
      content: prompt.trim(),
    };

    const newConversation = [...conversation, userMessage];
    setConversation(newConversation);
    setPrompt("");

    try {
      setLoading?.(true);
      const result = await chat(newConversation);

      const assistantMessage: ChatMessage = {
        role: "assistant",
        content: result,
      };
      setConversation([...newConversation, assistantMessage]);
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Failed to get response";
      onError?.(errorMessage);
    } finally {
      setLoading?.(false);
    }
  };

  const clearConversation = () => {
    setConversation([]);
  };

  return (
    <div className="mx-auto max-w-4xl rounded-lg bg-white p-8 shadow-lg">
      <div className="mb-6 flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-800">LLM Chat Demo</h2>
        {conversation.length > 0 && (
          <button
            onClick={clearConversation}
            className="rounded-md bg-gray-200 px-4 py-2 text-sm text-gray-700 transition-colors hover:bg-gray-300"
          >
            Clear Chat
          </button>
        )}
      </div>

      {/* Conversation History */}
      {conversation.length > 0 && (
        <div className="mb-6 max-h-96 space-y-4 overflow-y-auto rounded-lg bg-gray-50 p-4">
          {conversation.map((message, index) => (
            <div
              key={index}
              className={`flex ${message.role === "user" ? "justify-end" : "justify-start"}`}
            >
              <div
                className={`max-w-xs rounded-lg px-4 py-2 lg:max-w-md ${
                  message.role === "user"
                    ? "bg-blue-600 text-white"
                    : "border border-gray-200 bg-white text-gray-800"
                }`}
              >
                <p className="text-sm whitespace-pre-wrap">{message.content}</p>
              </div>
            </div>
          ))}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label
            htmlFor="prompt"
            className="mb-2 block text-sm font-medium text-gray-700"
          >
            Your Message
          </label>
          <textarea
            id="prompt"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Enter your message or question..."
            className="h-32 w-full resize-none rounded-md border border-gray-300 px-3 py-2 focus:border-transparent focus:ring-2 focus:ring-blue-500"
            required
          />
        </div>

        <button
          type="submit"
          className="w-full rounded-md bg-blue-600 px-4 py-2 text-white transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
          disabled={!prompt.trim()}
        >
          Send Message
        </button>
      </form>
    </div>
  );
}
