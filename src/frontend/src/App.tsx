import React, { useState } from "react";
import {
  CounterView,
  GreetingView,
  LlmPromptView,
  SupabaseDemo,
} from "./views";

interface Tab {
  id: string;
  label: string;
  icon: string;
}

const tabs: Tab[] = [
  { id: "counter", label: "Counter", icon: "🔢" },
  { id: "greeting", label: "Greeting", icon: "👋" },
  { id: "llm", label: "LLM Chat", icon: "🤖" },
  { id: "supabase", label: "Natural Query", icon: "🔍" },
];

function App(): React.JSX.Element {
  const [activeTab, setActiveTab] = useState<string>("supabase");
  const [error, setError] = useState<string>("");
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
    setTimeout(() => setError(""), 5000);
  };

  const renderActiveView = () => {
    const viewProps = {
      onError: handleError,
      setLoading: setIsLoading,
    };

    switch (activeTab) {
      case "counter":
        return <CounterView {...viewProps} />;
      case "greeting":
        return <GreetingView {...viewProps} />;
      case "llm":
        return <LlmPromptView {...viewProps} />;
      case "supabase":
        return <SupabaseDemo {...viewProps} />;
      default:
        return <CounterView {...viewProps} />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <header className="border-b bg-white shadow-sm">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="flex h-16 items-center justify-between">
            <div className="flex items-center space-x-4">
              <h1 className="text-2xl font-bold text-gray-900">IC Vibe</h1>
              <span className="text-sm text-gray-500">
                Internet Computer Demo
              </span>
            </div>

            {isLoading && (
              <div className="flex items-center space-x-2 text-blue-600">
                <div className="h-4 w-4 animate-spin rounded-full border-b-2 border-blue-600"></div>
                <span className="text-sm">Loading...</span>
              </div>
            )}
          </div>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className="border-b bg-white">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="flex space-x-8">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`border-b-2 px-1 py-4 text-sm font-medium transition-colors ${
                  activeTab === tab.id
                    ? "border-blue-500 text-blue-600"
                    : "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
                }`}
              >
                <span className="mr-2">{tab.icon}</span>
                {tab.label}
              </button>
            ))}
          </div>
        </div>
      </nav>

      {/* Error Banner */}
      {error && (
        <div className="border-l-4 border-red-400 bg-red-50 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <span className="text-red-400">⚠️</span>
            </div>
            <div className="ml-3">
              <p className="text-sm text-red-700">{error}</p>
            </div>
          </div>
        </div>
      )}

      {/* Main Content */}
      <main className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
        {renderActiveView()}
      </main>
    </div>
  );
}

export default App;
