import React from "react";
import { NaturalLanguageQuery } from "../components/NaturalLanguageQuery";

interface SupabaseDemoProps {
  onError?: (error: string) => void;
  setLoading?: (loading: boolean) => void;
}

export function SupabaseDemo({
  onError,
  setLoading,
}: SupabaseDemoProps): React.JSX.Element {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-6">
      <div className="mx-auto max-w-7xl">
        <NaturalLanguageQuery onError={onError} setLoading={setLoading} />
      </div>
    </div>
  );
}
