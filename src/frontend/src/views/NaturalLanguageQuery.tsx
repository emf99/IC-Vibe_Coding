import React, { useState } from "react";
import {
  querySupabaseWithNaturalLanguage,
  parseNaturalLanguageQuery,
} from "../services/backendService";

interface DataItem {
  id?: number;
  title?: string;
  is_done?: boolean;
  name?: string;
  email?: string;
  content?: string;
  user_id?: string;
  created_at?: string;
  due_date?: string;
}

interface NaturalLanguageQueryProps {
  onError?: (error: string) => void;
  setLoading?: (loading: boolean) => void;
}

export function NaturalLanguageQuery({
  onError,
  setLoading,
}: NaturalLanguageQueryProps) {
  const [query, setQuery] = useState("");
  const [data, setData] = useState<DataItem[]>([]);
  const [lastQuery, setLastQuery] = useState("");
  const [parseResult, setParseResult] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);

  const exampleQueries = [
    "get all todos",
    "show completed todos",
    "find incomplete todos",
    "get todos that are done",
    "show me unfinished tasks",
    "list all users",
    "find todos with id 1",
    "show me all posts",
  ];

  const handleQuery = async (e: React.FormEvent): Promise<void> => {
    e.preventDefault();
    if (!query.trim()) return;

    setIsLoading(true);
    setLoading?.(true);
    setLastQuery(query);

    try {
      // Show what the query parses to
      const parsed = await parseNaturalLanguageQuery(query);
      setParseResult(`Table: ${parsed.table}, Query: ${parsed.query}`);

      // Execute the actual query
      const result = await querySupabaseWithNaturalLanguage(query);
      setData(result);
      setQuery("");
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Unknown error occurred";
      onError?.(errorMessage);
      setData([]);
      setParseResult("");
    } finally {
      setIsLoading(false);
      setLoading?.(false);
    }
  };

  const handleExampleClick = (exampleQuery: string) => {
    setQuery(exampleQuery);
  };

  const formatValue = (key: string, value: any): React.ReactNode => {
    if (typeof value === "boolean") {
      return (
        <span
          className={`inline-flex rounded-full px-2 py-1 text-xs font-semibold ${
            value ? "bg-green-100 text-green-800" : "bg-red-100 text-red-800"
          }`}
        >
          {key === "is_done"
            ? value
              ? "Completed"
              : "Not Done"
            : value
              ? "Yes"
              : "No"}
        </span>
      );
    }

    if (
      typeof value === "string" &&
      (key.includes("created_at") || key.includes("due_date"))
    ) {
      return new Date(value).toLocaleString();
    }

    return String(value);
  };

  return (
    <div className="rounded-lg bg-white p-6 shadow-lg">
      <div className="mb-6">
        <h3 className="mb-2 text-2xl font-bold text-gray-800">
          Natural Language Database Query
        </h3>
        <div className="rounded-lg border border-blue-200 bg-blue-50 px-4 py-3 text-blue-700">
          <p className="font-medium">✨ AI-Powered Query Parser</p>
          <p className="mt-1 text-sm">
            Ask questions about your data in plain English and get structured
            results.
          </p>
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Query Input */}
        <div className="lg:col-span-1">
          <div className="rounded-lg bg-gray-50 p-4">
            <h4 className="mb-4 text-lg font-semibold text-gray-700">
              Ask Your Question
            </h4>

            <form onSubmit={handleQuery} className="space-y-4">
              <div>
                <label
                  htmlFor="query"
                  className="mb-2 block text-sm font-medium text-gray-700"
                >
                  Natural Language Query
                </label>
                <textarea
                  id="query"
                  placeholder="e.g., show me completed todos"
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  className="h-20 w-full resize-none rounded-md border border-gray-300 p-3 focus:border-transparent focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>

              <button
                type="submit"
                disabled={isLoading || !query.trim()}
                className="w-full rounded-md bg-blue-600 px-4 py-3 font-medium text-white transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
              >
                {isLoading ? "Processing..." : "Execute Query"}
              </button>
            </form>

            {/* Parse Result */}
            {parseResult && (
              <div className="mt-4 rounded-lg border border-yellow-200 bg-yellow-50 p-3">
                <p className="mb-1 text-xs font-medium text-yellow-800">
                  Parsed Query:
                </p>
                <p className="text-sm text-yellow-700">{parseResult}</p>
              </div>
            )}

            {/* Example Queries */}
            <div className="mt-6">
              <h5 className="mb-3 text-sm font-medium text-gray-700">
                Try These Examples:
              </h5>
              <div className="space-y-2">
                {exampleQueries.map((example, index) => (
                  <button
                    key={index}
                    onClick={() => handleExampleClick(example)}
                    className="w-full rounded p-2 text-left text-sm text-blue-600 transition-colors hover:bg-blue-50 hover:text-blue-800"
                  >
                    "{example}"
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Results Display */}
        <div className="lg:col-span-2">
          <div className="rounded-lg bg-gray-50 p-4">
            <div className="mb-4 flex items-center justify-between">
              <div>
                <h4 className="text-lg font-semibold text-gray-700">
                  Query Results
                </h4>
                {lastQuery && (
                  <p className="mt-1 text-sm text-gray-500">"{lastQuery}"</p>
                )}
              </div>
              <div className="text-sm text-gray-500">
                {data.length} record{data.length !== 1 ? "s" : ""} found
              </div>
            </div>

            {isLoading ? (
              <div className="flex items-center justify-center py-12">
                <div className="h-8 w-8 animate-spin rounded-full border-b-2 border-blue-600"></div>
                <span className="ml-3 text-blue-600">
                  Processing your query...
                </span>
              </div>
            ) : data.length > 0 ? (
              <div className="max-h-96 space-y-3 overflow-y-auto">
                {data.map((item, index) => (
                  <div
                    key={index}
                    className="rounded-md border border-gray-200 bg-white p-4 shadow-sm"
                  >
                    <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
                      {Object.entries(item).map(([key, value]) => (
                        <div key={key} className="flex flex-col">
                          <span className="text-xs font-medium tracking-wide text-gray-500 uppercase">
                            {key.replace("_", " ")}
                          </span>
                          <span className="mt-1 text-sm text-gray-800">
                            {formatValue(key, value)}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            ) : lastQuery ? (
              <div className="py-12 text-center text-gray-500">
                <div className="mb-4 text-4xl">🔍</div>
                <p className="text-lg">No results found</p>
                <p className="mt-1 text-sm">
                  Try a different query or check if the data exists
                </p>
              </div>
            ) : (
              <div className="py-12 text-center text-gray-500">
                <div className="mb-4 text-4xl">💬</div>
                <p className="text-lg">Ask a question to see results</p>
                <p className="mt-1 text-sm">
                  The AI will intelligently parse your natural language
                </p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
